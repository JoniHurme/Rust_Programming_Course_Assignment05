mod parcel;

fn main() {


    let package1 = parcel::ParcelStatus::Preparing;

    parcel::status_message(package1);

    let package2 = parcel::ParcelStatus::InTransit {
        courier: "DHL".to_string()
    };

    parcel::status_message(package2);

    parcel::print_delivery_note(Option::Some("Sometghin"))

    // println!("Package 1 is now in a state of:");

    // println!("Hello, world!");
}
