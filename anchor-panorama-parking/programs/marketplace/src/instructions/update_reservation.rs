use anchor_lang::{prelude::*, system_program::{Transfer, transfer}};


use crate::{ state::{Listing, Marketplace, ParkingSpaceStatus}};
use crate::error::ErrorCode;


#[derive(Accounts)]
pub struct UpdateReservation<'info> {
   #[account(mut)]
   pub renter: Signer<'info>,
   #[account(mut)]
   pub maker: SystemAccount<'info>, 
   
    #[account(
       
        seeds = [b"marketplace", marketplace.name.as_bytes()], 
        bump, 
    )]
    pub marketplace: Account<'info, Marketplace>,

    #[account(
        mut,
       seeds = [marketplace.key().as_ref(), 
        maker.key().as_ref()
        ], 
        bump,
        constraint = listing.parking_space_status == ParkingSpaceStatus::Available
        )]
    pub listing: Account<'info, Listing>,   

    pub system_program: Program<'info, System>,

}

impl <'info> UpdateReservation<'info> {
    pub fn update_reservation(&mut self, start_time: i64, end_time: i64) -> Result<()> {

        let listing = &mut self.listing;

        //does driver have enough funds?
        let duration = end_time - start_time;
        let rate_per_hour:i64 = listing.rental_rate.into();
        
        let driver_has_sufficient_funds:bool = self.renter.to_account_info().lamports() >= ((duration / 3600) * rate_per_hour).try_into().unwrap();

        if !driver_has_sufficient_funds {
            return Err(ErrorCode::InsufficientFunds.into());
        }
    
        if listing.parking_space_status != ParkingSpaceStatus::Available {
            return Err(ErrorCode::ListingNotAvailable.into());
        }
    
        listing.reserved_by = Some(self.renter.key());
        listing.reservation_start = Some(start_time);
        listing.reservation_end = Some(end_time);
        listing.parking_space_status = ParkingSpaceStatus::Reserved;

        
        //msg driver with reservation info, user story2B

         // Message to driver
    msg!("You updated your reservation, the parking space status is: {:?}", listing.parking_space_status);
    msg!("Reservation details: Start Time: {}, End Time: {}", start_time, end_time);

    // Message to homeowner that space is reserved
    let homeowner_msg = format!(
        "Your parking space has been updated by {} from {} to {}.",
        self.renter.key(),
        start_time,
        end_time
    );
    msg!("{}", homeowner_msg);

    //send a message to the driver with reservation info
    let driver_msg = format!(
        "Reservation updated! Your parking space is reserved from {} to {}.",
        start_time,
        end_time
    );
    msg!("{}", driver_msg);

      Ok(())
    }

    
}

