use std::time::Duration;

use tokio::{task, time::interval};
use uuid::Uuid;

use crate::{
    app::{fcm::models::fcm_message::FcmMessage, models::app_state::AppState, util::time},
    devices::{self, dtos::get_devices_filter_dto::GetDevicesFilterDto},
    reminders::{dtos::get_reminders_filter_dto::GetRemindersFilterDto, service},
};

use super::models::reminder::Reminder;

pub fn spawn(state: AppState) {
    task::spawn(async move {
        let mut interval = interval(Duration::from_secs(60));

        loop {
            interval.tick().await;

            let state = state.clone();
            task::spawn(async move {
                poll_reminders(&state).await;
            });
        }
    });
}

async fn poll_reminders(state: &AppState) {
    tracing::info!("polling reminders");
    let current_time_in_millis = time::current_time_in_millis();

    let dto = GetRemindersFilterDto {
        id: None,
        user_id: None,
        search: None,
        sort: Some("trigger_at,asc".to_string()),
        cursor: Some(format!(
            "{},{}",
            current_time_in_millis,
            Uuid::new_v4().to_string()
        )),
        limit: Some(100),
    };

    let Ok(reminders) = service::get_reminders(&dto, None, state).await else {
        return;
    };

    tracing::info!("{:?}", reminders);

    for reminder in reminders {
        if reminder.trigger_at > current_time_in_millis + 60000 {
            continue;
        }

        let _ = trigger_reminder(reminder, state).await;
    }
}

async fn trigger_reminder(reminder: Reminder, state: &AppState) {
    tracing::info!("trigger_reminder");

    let dto = GetDevicesFilterDto {
        id: None,
        user_id: Some(reminder.user_id.to_string()),
        sort: None,
        cursor: None,
        limit: None,
    };
    let Ok(devices) = devices::service::get_devices(&dto, None, state).await else {
        return;
    };

    let _fcm_client = state.authman.fcm_client(&state.http_client).await;
    let fcm_client = _fcm_client.read().await;

    for device in devices {
        tracing::info!("sending notification");

        if device.messaging_token.is_none() {
            continue;
        }

        let message = FcmMessage {
            token: device.messaging_token.unwrap(),
            title: reminder.title.to_owned().unwrap_or("Perroquet".to_string()),
            body: reminder.body.to_string(),
            click_action: None,
        };

        let _ = fcm_client.send(message, &state.http_client).await;
    }
}
