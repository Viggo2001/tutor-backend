use crate::modules::profile_management::tutor::{model::{PartialTutor, Tutor}};

pub async fn find_by_id(tutor_id: &str) -> Option<Tutor> {
    Tutor::find_by_id(tutor_id).await.unwrap()
}

pub async fn add_new(new_tutor: Tutor) -> Option<Tutor> {
    let result = Tutor::create(new_tutor).await;

    match result {
        Ok(tutor) => {
            Some(tutor)
        },
        Err(err) => {
            eprint!("Failed to add tutor: {}", err);
            None
        }
    }
}

pub async fn update(update_request: PartialTutor) -> u8 {
    
    if let Some(is_available) = update_request.is_available {
        // Handle update to is_available
        println!("Updating is_available to: {}", is_available);
    }

    if let Some(city) = update_request.city {
        if !city.trim().is_empty() {
            println!("Updating city to: {}", city);
        }
    }

    if let Some(country) = update_request.country {
        if !country.trim().is_empty() {
            println!("Updating country to: {}", country);
        }
    }

    if let Some(year) = update_request.year {
        // Handle update to year
        print!("Updating year to: {}", year);
    }

    if let Some(bio) = update_request.bio {
        if !bio.trim().is_empty() {
            println!("Updating bio to: {}", bio);
        }
    }

    if let Some(average) = update_request.average {
        println!("Updating average to: {}", average);
    }

    0
}

