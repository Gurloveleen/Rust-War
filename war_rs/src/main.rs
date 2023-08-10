#![allow(non_snake_case,non_camel_case_types,dead_code)]

/*
    Below is the function stub for deal. Add as many helper functions
    as you like, but the deal function should not be modified. Just
    fill it in.
    
    Test your code by running 'cargo test' from the war_rs directory.
*/

fn deal(shuf: &[u8; 52]) -> [u8; 52]
{
    let mut pile1 = Vec::new(); //Player 1 Deck (Vector)
    let mut pile2 = Vec::new(); // Player 2 Deck (Vector)
    let mut pile3 = Vec::new(); // Temporary deck (Vector)
    for i in 0..shuf.len() //Distributing Cards among the players
    {
        if i%2==0
        {
            pile1.push(shuf[i]);
        }
        else
         {
            pile2.push(shuf[i]);
        }
    }
    pile1.reverse(); // Reversing the decks
    pile2.reverse();
    for i in pile1.iter_mut() // Replacing all the 1's to 14's
     {
        if *i == 1 
        {
            *i = 14;
        }
    }
    for i in pile2.iter_mut() 
    {
        if *i == 1
         {
            *i = 14;
        }
    }
            while pile1.len()>0 && pile2.len()>0 // Starting the game
            {
            
            if pile1[0]>pile2[0] // Checking if first card of Player1 is greater than first card of Player2
            {
                pile3.push(pile1[0]);
                pile3.push(pile2[0]);
                pile3.sort_by(|a, b| b.cmp(a));
                pile1.extend(pile3.clone());
                pile3.clear();
            }
            else if pile1[0]<pile2[0] // Checking if first card of Player2 is greater than first card of Player1
            {
                pile3.push(pile1[0]);
                pile3.push(pile2[0]);
                pile3.sort_by(|a, b| b.cmp(a));
                pile2.extend(pile3.clone());
                pile3.clear();
            }
            else if pile1.len()>=2 && pile2.len()>=2 // Case where War Happens and both players have atleast 2 cards
                {
                    pile3.push(pile1[0]);
                    pile3.push(pile2[0]);
                    pile3.push(pile1[1]);
                    pile3.push(pile2[1]);
                    pile3.sort_by(|a, b| b.cmp(a));
                    pile1.remove(1); // Removing Face down cards from both the piles
                    pile2.remove(1);

                }
                else if pile1.len()==1 && pile2.len()>1 // Checking for case where Player1 has only 1 card left and War happens
                {
                    if pile1[0]==pile2[0]
                    {
                        pile2.push(pile1[0]);
                        pile2.push(pile2[0]);

                    }
                }
                else if pile2.len()==1 && pile1.len()>1 // Checking for case where Player2 has only 1 card left and War happens
                {
                    if pile1[0]==pile2[0]
                    {
                        pile1.push(pile1[0]);
                        pile1.push(pile2[0]);
                    }
                }
            

                pile1.remove(0); //Removing the first card from both the piles
                pile2.remove(0);
        }
        
        if pile1.len()==0 && pile2.len()==0 // Case where there is a tie
        {
            pile3.sort_by(|a, b| b.cmp(a));
            let mut p3=[0;52]; 
            for i in pile3.iter_mut() // Replacing all the 14's to 1's
            {
                if *i == 14 
                {
                    *i = 1;
                }
            }
            for i in 0..pile3.len() // Copying all the cards to array from vector since the return type is array
            {
             p3[i]=pile3[i];
            }
            p3 // Returning the Winning deck
        }
        else if pile1.len()==0 // Case where Player2 Wins
        {
            pile3.sort_by(|a, b| b.cmp(a));
            pile2.extend(pile3.clone()); 
            let mut p2=[0;52]; 
            for i in pile2.iter_mut() // Replacing all the 14's to 1's
             {
                if *i == 14
                 {
                    *i = 1;
                }
            }
            for i in 0..pile2.len() // Copying all the cards to array from vector since the return type is array
            {
             p2[i]=pile2[i];
            }
            p2 // Returning the Winning deck
        }
        else  // Case where Player1 Wins
        {
            pile3.sort_by(|a, b| b.cmp(a));
            pile1.extend(pile3.clone()); 
            let mut p1=[0;52]; 
            for i in pile1.iter_mut() // Replacing all the 1's to 14's
             {
                if *i == 14 
                {
                    *i = 1;
                }
            }
            for i in 0..pile1.len() // Copying all the cards to array from vector since the return type is array
            {
             p1[i]=pile1[i];
            }
            p1 // Returning the Winning deck
            
        }
}


    


#[cfg(test)]
#[path = "tests.rs"]
mod tests;

