use crate::parcel::ParcelStatus;
use crate::parcel::ParcelStatus::Delivered;

mod parcel;

fn main() {



    let package1 = parcel::ParcelStatus::Preparing;
    let package2 = parcel::ParcelStatus::InTransit {
        courier: "DHL".to_string()
    };
    let package3 = parcel::ParcelStatus::Delivered {
        recipient: "Front Desk".to_string()
    };
    let package4 = parcel::ParcelStatus::Returned {
        reason: "the goods were broken.".to_string()
    };

    let package_array = [package1, package2, package3, package4];

    for package in package_array {
        println!("{}", parcel::status_message(package));
    }

    parcel::print_delivery_note(
        Option::Some("Delivered")
    );
    // Should output "The note is none"
    parcel::print_delivery_note(
        Option::None
    );

    parcel::announce_delivery(
        &Delivered { recipient: "Front".to_string() }
    );

    parcel::create_receipt(
        ParcelStatus::Delivered {recipient:"Front Desk".to_string()}
    );
}
