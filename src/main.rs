mod parcel;

fn main() {


    // Definition of packages with all variants of the enum.
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

    // array of parcels to be looped through to print the status_message.
    let package_array = [ package1, package2, package3, package4, ];

    for package in package_array {
        println!("{}", parcel::status_message(package));
    }


    // Parcel that is being prepared
    parcel::announce_delivery(&package1);
    parcel::create_receipt(package1);
    // Parcel that has been delivered
    parcel::announce_delivery(&package3);
    parcel::create_receipt(package3);


    // Should output something about the parcel delivery.
    parcel::print_delivery_note(
        Option::Some("Leave the parcel at reception.")
    );
    // Should output about the parcel not being delivered yet.
    parcel::print_delivery_note(
        Option::None
    );
}
