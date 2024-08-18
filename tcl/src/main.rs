use std::fs::OpenOptions;
use std::io::{self, BufRead, BufReader, Write};

//define ticket struct. 
//INCLUDE:
//description: String
//priority: u8
//status: String
//ticket_type: String
//estimated_duration: u32
//duration_value: String
struct Ticket {
    id: u32,
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
    id: u32,
    description: String,
    priority: u8,
    status: String,
    ticket_type: String,
    estimated_duration: u32,
    duration_value: String
) {
    //create a new ticket with the given values
    let ticket = Ticket {
        id,
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
    //iterate over the vector of tickets
    //enumerate() returns an iterator that yields the index of the element along with a reference to the element itself
    //i is the index of the element, ticket is a reference to the element
    //using i + 1 to start the ticket numbering at 1 instead of 0
    //.iter() returns an iterator over the vector
    for (i, ticket) in tickets.iter().enumerate() {
        println!(
            //print out the ticket's id, description, priority, status, type, estimated duration, and duration value
            //{} is a placeholder for the values that will be inserted into the string
            //i + 1 is the ticket number
            "{}: [{}] {} (Priority: {}, Status: {}, Type: {}, Estimated Duration: {}, Duration Value: {})",
            ticket.id,
            ticket.status,
            ticket.description,
            ticket.priority,
            ticket.status,
            ticket.ticket_type,
            ticket.estimated_duration,
            ticket.duration_value,
        );
    }
}

//UPDATE TICKET FUNCTION
//takes in a mutable reference to a vector of tickets, an index, a new status, a new description, a new priority, a new ticket type, a new estimated duration, and a new duration value
fn update_ticket(tickets:
    &mut Vec<Ticket>,
    index: usize,
    new_status:String,
    new_description: String,
    new_priority: u8,
    new_ticket_type: String,
    new_estimated_duration: u32,
    new_duration_value: String
) {
    //check if the index is within the bounds of the vector
    if index < tickets.len() {
        //update the ticket at the specified index with the new values
        tickets[index].status = new_status;
        /*
        if let is used to handle the case where the new values are None
        if the new value is Some, update the ticket with the new value
        if the new value is None, do nothing
        this allows us to update only the fields that have new values specified
        */
        if let Some(description) = new_description {
            /*
            update the ticket's description with the new value
            the new_description parameter is an Option<String>, so we need to unwrap it to get the actual String value
            if the new_description is None, this block will not be executed
            if the new_description is Some, the description field of the ticket will be updated with the new value
            the same pattern is used for the other fields
            */
            tickets[index].description = description;
        } if let Some(priority) = new_priority {
            tickets[index].priority = priority;
        } if let Some(ticket_type) = new_ticket_type {
            tickets[index].ticket_type = ticket_type;
        } if let Some(estimated_duration) = new_estimated_duration {
            tickets[index].estimated_duration = estimated_duration;
        } if let Some(duration_value) = new_duration_value {
            tickets[index].duration_value = duration_value;
        } else {
            println!("Invalid ticket. Cannot perform update operation.");
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
fn save_tickets(tickets: &Vec<Ticket>, index: usize){
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
        //write the ticket's id, description, priority, status, type, estimated duration, and duration value to the file
        writeln!(
            file,
            "{}:{}:{}:{}:{}:{}:{}:{}",
            ticket.id,
            ticket.status,
            ticket.description,
            ticket.priority,
            ticket.status,
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
    Ok(())
}

fn main() {
    println!("Hello, world!");
}
