mod parcel;

fn main() {


    // Definition of packages with all variants of the enum.
    let package1 = parcel::ParcelStatus::Preparing;
    // Extra package with status Preparing to work around ownership caused issues.
    // fn status:message takes ownership of the package so it can't be used again after that.
    let package12 = parcel::ParcelStatus::Preparing;
    let package2 = parcel::ParcelStatus::InTransit {
        courier: "DHL".to_string()
    };
    // Extra package with status Preparing to work around ownership caused issues.
    let package32 = parcel::ParcelStatus::Delivered {
        // recipient: "Front Desk".to_string()
        recipient: String::from("Front Desk")
    };
    let package3 = parcel::ParcelStatus::Delivered {
        recipient: "Front Desk".to_string()
    };
    let package4 = parcel::ParcelStatus::Returned {
        reason: "the goods were broken.".to_string()
    };

    // Create values for at least Preparing, InTransit, Delivered, and
    // Returned.
    // array of parcels to be looped through to print the status_message.
    let package_array = [ package1, package2, package3, package4, ];

    for package in package_array {
        println!("{}", parcel::status_message(package));
    }

    // Call announce_delivery with both a delivered and a non-delivered parcel.
    // Call create_receipt with both a delivered and a non-delivered parcel.
    // Parcel that is being prepared
    parcel::announce_delivery(&package12);
    parcel::create_receipt(package12);
    // Parcel that has been delivered
    parcel::announce_delivery(&package32);
    parcel::create_receipt(package32);

    // Call print_delivery_note once with Some(...) and once with None.
    // Should output something about the parcel delivery.
    parcel::print_delivery_note(
        Option::Some("Leave the parcel at reception.")
    );
    // Should output about the parcel not being delivered yet.
    parcel::print_delivery_note(
        Option::None
    );
}
