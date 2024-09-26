use core::num;
use std::fs::OpenOptions;
use std::io::{self, BufRead, BufReader, Write};
use std::process::id;
use std::u8;
use rand::Rng;

//define ticket struct. 
//INCLUDE:
//description: String
//priority: u8
//status: String
//ticket_type: String
//estimated_duration: u32
//duration_value: String
struct Ticket {
    tixid: u32,
    description: String, 
    priority: u8,
    status: String,
    ticket_type: String,
    estimated_duration: u32,
    duration_value: String,
}

//CREATE TICKET FUNCTION
//takes in a mutable reference to a vector of tickets, a description, a priority, a status, a ticket type, an estimated duration, and a duration value
//creates a new ticket with the given values and pushes it onto the vector
fn create_ticket(
    //mutable reference to a vector of tickets
    tickets: &mut Vec<Ticket>,
    tixid: u32,
    description: String,
    priority: u8,
    status: String,
    ticket_type: String,
    estimated_duration: u32,
    duration_value: String
) {
    //create a new ticket with the given values
    let ticket = Ticket {
        tixid,
        description,
        priority,
        status,
        ticket_type,
        estimated_duration,
        duration_value,
    };
    //push the new ticket onto the vector
    tickets.push(ticket);
}

//LIST TICKETS FUNCTION
//takes in a reference to a vector of tickets
//iterates over the vector and prints out each ticket's description, priority, status, type, estimated duration, and duration value
fn list_tickets(tickets: &Vec<Ticket>) {
    let saved_tickets = load_tickets("tickets.txt").unwrap_or_default();
    //iterate over the vector of tickets
    //enumerate() returns an iterator that yields the index of the element along with a reference to the element itself
    //i is the index of the element, ticket is a reference to the element
    //using i + 1 to start the ticket numbering at 1 instead of 0
    //.iter() returns an iterator over the vector
    println!("================================================================");
    println!("====TICKETS IN MEMORY===================TICKETS IN MEMORY=======");
    println!("================================================================");
    for (i, ticket) in tickets.iter().enumerate() {
        println!(
            //print out the ticket's tixid, description, priority, status, type, estimated duration, and duration value
            //{} is a placeholder for the values that will be inserted into the string
            //i + 1 is the ticket number
            "{}: [{}] - {} - (Priority: {}, Type: {}, Estimated Duration: {} {})",
            ticket.tixid,
            ticket.status.to_uppercase(),
            ticket.description,
            ticket.priority,
            ticket.ticket_type.to_uppercase(),
            ticket.estimated_duration,
            ticket.duration_value.to_uppercase(),
        );
    }
    println!("================================================================");
    println!("====TICKETS IN MEMORY===================TICKETS IN MEMORY=======");
    println!("================================================================");

    println!("================================================================");
    println!("====TICKETS IN FILES=====================TICKETS IN FILES=======");
    println!("================================================================");
    for (i, ticket) in saved_tickets.iter().enumerate(){
        println!(
            "{}: [{}] - {} - (Priority: {}, Type: {}, Estimated Duration: {} {})",
            i + 1 + tickets.len(),
            ticket.status.to_uppercase(),
            ticket.description,
            ticket.priority,
            ticket.ticket_type.to_uppercase(),
            ticket.estimated_duration,
            ticket.duration_value.to_uppercase(),
        );
    }
    println!("================================================================");
    println!("====TICKETS IN FILES=====================TICKETS IN FILES=======");
    println!("================================================================");
}

//UPDATE TICKET FUNCTION
//takes in a mutable reference to a vector of tickets, an index, a new status, a new description, a new priority, a new ticket type, a new estimated duration, and a new duration value
fn update_ticket(
    tickets: &mut Vec<Ticket>,
    tixid: u32,
    new_status: Option<String>,
    new_description: Option<String>,
    new_priority: Option<u8>,
    /*new_ticket_type: Option<String>,
    new_estimated_duration: Option<u32>,
    new_duration_value: Option<String>*/
) { for ticket  in tickets {
    if ticket.tixid == tixid {
        //update the ticket at the specified index with the new values
        if let Some(ref status) = new_status {
            ticket.status = status.to_string();
        }
        /*
        if let is used to handle the case where the new values are None
        if the new value is Some, update the ticket with the new value
        if the new value is None, do nothing
        this allows us to update only the fields that have new values specified
        */
        if let Some(ref description) = new_description {
            /*
            update the ticket's description with the new value
            the new_description parameter is an Option<String>, so we need to unwrap it to get the actual String value
            if the new_description is None, this block will not be executed
            if the new_description is Some, the description field of the ticket will be updated with the new value
            the same pattern is used for the other fields
            */
            ticket.description = description.to_string();
        } if let Some(ref priority) = new_priority {
            ticket.priority = *priority;
        } /* if let Some(ticket_type) = new_ticket_type {
            tickets[index].ticket_type = ticket_type;
        } if let Some(estimated_duration) = new_estimated_duration {
            tickets[index].estimated_duration = estimated_duration;
        } if let Some(duration_value) = new_duration_value {
            tickets[index].duration_value = duration_value;
        } */ else {
            println!("Invalid ticket. Cannot perform update operation.");
        }
    }
}
}



/*
REMOVE TICKET FUNCTION
takes in a mutable reference to a vector of tickets and an index
removes the ticket at the specified index from the vector
if the index is out of bounds, prints an error message
*/
fn remove_ticket(tickets: &mut Vec<Ticket>, index: usize) {
    //check if the index is within the bounds of the vector
    //if it is, remove the ticket at the specified index
    if index < tickets.len() {
        tickets.remove(index);
    } else {
        println!("Invalid ticket. Cannot perform removal operation.");
    }
}


/*
SAVE TICKETS FUNCTION
takes in a reference to a vector of tickets and a filename
opens the specified file for writing
*/
fn save_tickets(tickets: &Vec<Ticket>, filename: &str) -> core::result::Result<(), std::io::Error> {
    /*
    open the file with the specified filename
    if the file does not exist, it will be created
    if the file already exists, its contents will be truncated
    the file is opened in write mode
    */
    let mut file = OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open(filename)?;

    for ticket in tickets {
        //write the ticket's tixid, description, priority, status, type, estimated duration, and duration value to the file
        writeln!(
            file,
            "{}:{}:{}:{}:{}:{}:{}",
            ticket.tixid,
            ticket.status,
            ticket.description,
            ticket.priority,
            ticket.ticket_type,
            ticket.estimated_duration,
            ticket.duration_value,
        )?;
    }
    /*
    flush the file to ensure that all data is written to disk
    this is necessary because writes to files are buffered for performance reasons
    flushing the file ensures that all data is written before the function returns
    if the program crashes before the file is flushed, some data may be lost!!!
    */
    file.flush()?;
    Ok(())
    /*TO UNWRAP/EXPECT RESULT (not recommended):
        save_tickets(&tickets, "tickets.txt").unwrap();
        or with a custom error message
        save_tickets(&tickets, "tickets.txt").expect("Failed to save tickets");
     */
    /*TO PATTERN MATCH ON RESULT:
        match save_tickets(&tickets, "tickets.txt") {
            Ok(()) => println!("Tickets saved successfully"),
            Err(e) => eprintln!("Failed to save tickets: {}", e),
        }
    */
    /*MAP AND/OR THEN ERROR HANDLING for values and errors:
        save_tickets(&tickets, "tickets.txt")
            .map(|_| println!("Tickets saved successfully"))
            .map_err(|e| eprintln!("Failed to save tickets: {}", e));
     */
}


/*LOAD TICKETS FUNCTION
    iterate over a Vec of Tickets, unwrapping the ticket values, and pushing to 'tickets' to Return a list of Tickets.
*/
fn load_tickets(filename: &str) -> core::result::Result<Vec<Ticket>, std::io::Error> {
    let file = OpenOptions::new().read(true).open(filename)?;
    let reader = BufReader::new(file);
    let mut tickets = Vec::new();

    for line in reader.lines() {
        let line = line?;
        let parts: Vec<&str> = line.split(':').collect();

        if parts.len() == 7{
            let description = parts[0].to_string();
            let priority: u8 = parts[1].parse().unwrap_or(1);
            let status = parts[2].to_string();
            let ticket_type = parts[3].to_string();
            let estimated_duration: u32 = parts[4].parse().unwrap_or(0);
            let duration_value = parts[5].to_string();
            let tixid: u32 = parts[6].parse().unwrap_or(666);

            //add ticket (iter) to the list??
            tickets.push(Ticket{
                tixid,
                description,
                priority,
                status,
                ticket_type,
                estimated_duration,
                duration_value,
            });
        }
    }
    //return a list of tickets (result called above)
    Ok(tickets)
}

fn main() {
    let filename = "tcl/src/main.txt";
    println!("use this menu to select an option for operation. create a ticket, update a ticket, delete a ticket.");
    /*
    load existing tickets from file/disk
    display menu (UI) via looping println (each option will have  number)
    read user input in loop (user should input a corresponding menu number)
    parse input >>  validate input (number &str bool) (figure out menu option)
    match statement handling
    case 1: Create Ticket
    case 2: List Tickets
    case 3: Update Existing Ticket
    case 4: Delete Ticket
    case 5: Save Work to File & Exit Application
    */
    let mut tickets = match load_tickets(filename) {
        Ok(tickets) => tickets,
        Err(_) => Vec::new(),
    };

    loop {
        println!("================================================================");
        println!("====MENU========================MENU=================MENU=======");
        println!("================================================================");
        println!("1. Create Ticket(s)");
        println!("2. List Tickets");
        println!("3. Update Existing Ticket");
        println!("4. Delete Ticket");
        println!("5. Save Work to File & Exit Application");
        println!("================================================================");
        println!("==INPUT BELOW=======================INPUT BELOW=================");
        println!("================================================================");
        //utilize flush on standard out to ensure its appearance (immediately buffered contents meet their destination)
        io::stdout().flush().unwrap();

        let mut userinput = String::new();
        io::stdin().read_line(&mut userinput).expect("Failed to correctly read user input. Please try again.");
        let choice: u32 = userinput.trim().parse().expect("User input not recognized, please input a number corresponding to desired operation.");

        match choice {
            1 => {
                println!("================================================================");
                println!("Enter a ticket DESCRIPTION: ");
                println!("================================================================");
                let mut description = String::new();
                io::stdin().read_line(&mut description).expect("Failed to read line.");
                
                println!("================================================================");
                println!("Enter a ticket PRIORITY (1-5): ");
                println!("================================================================");
                let mut priority_str = String::new();
                io::stdin().read_line(&mut priority_str).expect("Failed to read line.");
                let priority: u8 = priority_str.trim().parse().expect("Please enter a valid u8 number (1-5).");

                println!("================================================================");
                println!("Enter a TICKET TYPE (Research | Development | BugFix): ");
                println!("================================================================");
                let mut ticket_type = String::new();
                io::stdin().read_line(&mut ticket_type).expect("Failed to read line.");

                println!("================================================================");
                println!("Enter a ticket DURATION value: ");
                println!("================================================================");
                let mut estimated_duration_str = String::new();
                io::stdin().read_line(&mut estimated_duration_str).expect("Failed to read line.");
                let estimated_duration: u32 = estimated_duration_str.trim().parse().expect("Please enter a valid u32 number representing a time value for duration.");

                println!("================================================================");
                println!("Enter a ticket DURATION UNIT (e.g. Hours, Days): ");
                println!("================================================================");
                let mut duration_value = String::new();
                io::stdin().read_line(&mut duration_value).expect("Failed to read line.");

                println!("================================================================");
                println!("Enter a ticket STATUS (Open | In-Review | Closed): ");
                println!("================================================================");
                let mut status = String::new();
                io::stdin().read_line(&mut status).expect("Failed to read line.");

                let mut rng_id = rand::thread_rng();
                let tixid: u32 = rng_id.gen_range(100000..=999999);

                create_ticket(
                    &mut tickets,
                    tixid,
                    description.trim().to_string(),
                    priority,
                    status.trim().to_string(),
                    ticket_type.trim().to_string(),
                    estimated_duration,
                    duration_value.trim().to_string(),
                );
            },

            2 => list_tickets(&tickets),

            3 => {
                println!("Enter a Ticket Number to Update");
                let mut useridinput = String::new();
                io::stdin().read_line(&mut useridinput).expect("Failed to read line");
                let idmatch = useridinput.trim().parse::<u32>();
                
                match idmatch {
                    Ok(inputid) => {
                        let ticket_exists = tickets.iter().any(|ticket| ticket.tixid == inputid);
                        if ticket_exists {
                            println!("Ticket found! Ticket: {}", inputid);
                            println!("Now let's run through the Status, Description and Priority fields for update..");
                            println!("Update the ticket STATUS (Open | In-Review | Closed) (ENTER or RETURN without editing to keep the same Status): ");
                            let mut status = String::new();
                            io::stdin().read_line(&mut status).expect("Failed to read line.");

                            println!("Enter a new DESCRIPTION (ENTER or RETURN without editing to keep the same Description): ");
                            let mut description = String::new();
                            io::stdin().read_line(&mut description).expect("Failed to read line");

                            println!("Change Ticket PRIORITY (1-5) (ENTER or RETURN without editing to keep the same Priority): ");
                            let mut priority_input = String::new();
                            io::stdin().read_line(&mut priority_input).expect("Failed to read line.");
                            let trimmed_input = priority_input.trim();

                            let mut priority: Option<u8> = None;

                            if !trimmed_input.is_empty() {
                                match trimmed_input.parse::<u8>() {
                                    Ok(num) if num >= 1 && num <= 5 => {
                                        priority = Some(num);
                                    },
                                    _ => {
                                        println!("Failed to read a valid number between 1 and 5")
                                    }
                                }
                            }
                            

                            let new_status = if status.trim().is_empty() {
                                None
                            } else {
                                Some(status.trim().to_string())
                            };

                            let new_description = if description.trim().is_empty() {
                                None
                            } else {
                                Some(description.trim().to_string())
                            };

                            let new_priority = priority.map(|p| p);

                            let tixid = inputid;

                            update_ticket(&mut tickets,
                                tixid,
                                new_status,
                                new_description,
                                new_priority,
                            );
                        
                        } else {
                            println!("No ticket found with ID: {}", inputid);
                        }
                    },
                    Err(_) => println!("Invalid input. Please enter a valid ticket number. See the ticket list for active tickets.")
                }
                                    
            }, 
            4 => {
                println!("Enter a ticket number to remove: ");
                let mut num = String::new();
                io::stdin().read_line(&mut num).expect("Failed to read line.");
                let index: usize = num.trim().parse().expect("Please enter a valid numnber.");
                remove_ticket(&mut tickets, index);
            },
            5 => {
                //save all of the tickets thus made, exit loop
                save_tickets(&tickets, "tickets.txt").expect("Failed to save tickets.");
                break;
            }
            _ => {
                println!("Invalid input");
            }  
        }
    }
}
