use std::fs::OpenOptions;
use std::io::{self, BufRead, BufReader, Write};


struct Ticket {
    description: String, 
    priority: u8,
    status: String,
    ticket_type: String,
    estimated_duration: u32,
    duration_value: String,
}

fn create_ticket(tickets: &mut Vec<Ticket>, description: String, priority: u8, status: String, ticket_type: String, estimated_duration: u32, duration_value: String) {
    let ticket = Ticket {
        description,
        priority,
        status,
        ticket_type,
        estimated_duration,
        duration_value,
    };
    tickets.push(ticket);
}

fn list_tickets(tickets: &Vec<Ticket>) {
    for (i, ticket) in tickets.iter().enumerate() {
        println!(
            "{}: [{}] {} (Priority: {}, Status: {}, Type: {}, Estimated Duration: {}, Duration Value: {})",
            i + 1,
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

fn update_ticket(tickets: &mut Vec<Ticket>, index: usize, new_status:String, new_description: String, new_priority: u8, new_ticket_type: String, new_estimated_duration: u32, new_duration_value: String) {
    if index < tickets.len() {
        tickets[index].status = new_status;
        if let Some(description) = new_description {
            tickets[index].description = description;
        } else {
            println!("Invalid ticket number.");
        }
    }
}

fn remove_ticket(tickets: &mut Vec<Ticket>, index: usize) {
    if index < tickets.len() {
        tickets.remove(index);
    } else {
        println!("Invalid ticket number.");
    }
}

fn main() {
    println!("Hello, world!");
}
