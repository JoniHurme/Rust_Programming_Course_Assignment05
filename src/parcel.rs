use crate::parcel::ParcelStatus::Delivered;

// 1. Define Parcel Statuses
// Define an enum named ParcelStatus with these variants:
pub enum ParcelStatus {
    Preparing,
    InTransit {courier: String},
    Delivered {recipient: String},
    Returned {reason: String},
}

// Enum definition is ok.


// 2. Describe Every Status
// Implement a function named status_message with this signature:
// fn status_message(status: ParcelStatus) -> String
// Must be exhaustive. Is it so without an else block/clause whatever? ||| There are no more possibilities, so it is exhaustive.

pub fn status_message(status: ParcelStatus) -> String {
    match status {
        ParcelStatus::Preparing => "Parcel is preparing".to_string(),
        ParcelStatus::InTransit {courier} => format!("Parcel is in transit with courier {}", courier),
        ParcelStatus::Delivered {recipient} => format!("Parcel is delivered to {}", recipient),
        ParcelStatus::Returned {reason} => format!("Parcel is returned due to {}", reason),
    }
}

// OK


// 3. Print an Optional Delivery Note
// Implement a function named print_delivery_note with this signature:
// fn print_delivery_note(note: Option<&str>)
// ------- The function
// Use if let to print the delivery note when it is Some. Use an else block
// to print a message when the note is None.
pub fn print_delivery_note(note: Option<&str>) {

    if let Some(delivery_note) = note {
        println!("Delivery note: {}", delivery_note)
    } else {
        println!("The note is none")
    }
}

// OK


// 4. Announce Delivered Parcels
// Implement a function named announce_delivery with this signature:
// fn announce_delivery(status: &ParcelStatus)
// Use if let to check whether the status is ParcelStatus::Delivered. If it
// is, print the recipient's name. For every other status, print a message
// stating that the parcel has not been delivered yet.
pub fn announce_delivery(status: &ParcelStatus) {

    if let ParcelStatus::Delivered {recipient} = status {
        println!("works {}", recipient)
    } else {
        println!("No")
    }
}

// OK

    
// 5. Create a Delivery Receipt
// Implement a function named create_receipt with this signature:
// fn create_receipt(status: ParcelStatus)
pub fn create_receipt(status: ParcelStatus) {

    println!("\n output of create_receipt");
    //
    // let Some(status) = ParcelStatus::Delivered {recipient} else {
    //     return;
    // };
    // println!("works? {}", status)


    let ParcelStatus::Delivered{recipient} = status else {
        println!("{}", status);
        return;
    };
    // println!("{}", status)




    // let Some(ParcelStatus::Delivered {recipient}) = status {
    //
    // } else {
    //     println!("no")
    // };
    //

    // let Some(status) = (Delivered { recipient: String::from("")) } else {
    //     println!("No delivery yet.");
    //     return;
    // };
    // println!("{}", status);

}