use data_types_and_structures::{get_user_info, RequestStatus, User};
use std::collections::HashMap;

fn main() {
    // structs: Creating a new user
    let mut user = User::new(1, "Alice", "alice@example.com");
    println!("{} ({})", user.name, user.email); // Alice (alice@example.com)
    user.deactivate();

    // enums: Checking the status of a request
    let status = RequestStatus::Processing;
    println!("{}", status.message()); // Request is being processed

    // tuples: Extracting user information
    let (name, email) = get_user_info();
    println!("User: {} - {}", name, email);

    // arrays: Creating a new array
    let identifiers: [i32; 5] = [1, 2, 3, 4, 5];
    let _first = identifiers[0]; // 1
    let _last = identifiers[identifiers.len() - 1]; // 5
    for id in &identifiers {
        println!("{}", id);
    }

    // vectors: Creating a new vector
    let mut rating: Vec<i32> = vec![85, 92, 78];
    rating.push(88); // [85, 92, 78, 88]
    rating.sort(); // [78, 85, 88, 92]
    for value in &rating {
        println!("{}", value);
    }

    // hashmaps: Creating a new hashmap
    let mut team: HashMap<String, Vec<String>> = HashMap::new();
    team.insert(
        String::from("Engineering"),
        vec![String::from("Alice"), String::from("Bob")],
    );
    team.insert(
        String::from("Sales"),
        vec![String::from("Charlie"), String::from("David")],
    );

    if let Some(members) = team.get("Engineering") {
        println!("Engineering team members: {:?}", members);
    } else {
        println!("Engineering team not found");
    }

    for (team_name, members) in &team {
        println!("{} team members: {:?}", team_name, members);
    }
}

#[cfg(test)]
mod tests {
    use data_types_and_structures::{get_user_info, RequestStatus, User};
    use std::collections::HashMap;

    #[test]
    fn test_user_creation_and_deactivation() {
        let mut user = User::new(1, "Alice", "alice@example.com");
        assert_eq!(user.name, "Alice");
        assert_eq!(user.email, "alice@example.com");
        assert!(user.active);

        user.deactivate();
        assert!(!user.active);
    }

    #[test]
    fn test_request_status_message() {
        let status = RequestStatus::Processing;
        assert_eq!(status.message(), "Request is being processed");
    }

    #[test]
    fn test_get_user_info() {
        let (name, email) = get_user_info();
        assert_eq!(name, "Alice");
        assert_eq!(email, "alice@example.com");
    }

    #[test]
    fn test_array_operations() {
        let identifiers: [i32; 5] = [1, 2, 3, 4, 5];
        assert_eq!(identifiers[0], 1);
        assert_eq!(identifiers[identifiers.len() - 1], 5);

        let mut output = String::new();
        for id in &identifiers {
            output.push_str(&format!("{} ", id));
        }
        assert_eq!(output.trim(), "1 2 3 4 5");
    }

    #[test]
    fn test_vector_operations() {
        let mut rating: Vec<i32> = vec![85, 92, 78];
        rating.push(88);
        assert_eq!(rating, vec![85, 92, 78, 88]);

        rating.sort();
        assert_eq!(rating, vec![78, 85, 88, 92]);

        let mut output = String::new();
        for value in &rating {
            output.push_str(&format!("{} ", value));
        }
        assert_eq!(output.trim(), "78 85 88 92");
    }

    #[test]
    #[should_panic(expected = "Engineering team not found")]
    fn test_hashmap_operations() {
        let mut team: HashMap<String, Vec<String>> = HashMap::new();
        team.insert(
            String::from("Sales"),
            vec![String::from("Charlie"), String::from("David")],
        );

        if let Some(members) = team.get("Engineering") {
            assert_eq!(members, &vec![String::from("Alice"), String::from("Bob")]);
        } else {
            panic!("Engineering team not found");
        }
    }
}
