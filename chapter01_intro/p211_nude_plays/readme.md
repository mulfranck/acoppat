# Problem 1.2.11 Nude Plays
ticket price at 2.50
posible sale 120 tickets
expenses 80 cents per patron/tickets

i ticket reduct to 2.50 - 5
posible sale 120+ 15
expenses in by 2 per patron

1 cents = 1/100 dollar;
5 cents = x = 5*100 dollar

reduction of 5cents = 2.5 dollar - 5/100 dollar;
expenses goes by 2 cents ie 80cents + 2 cents
for 5 cent ie 5/100 dollar, he loses 2cents
so a gain per person is (2.5 dollar - 5/100 dollar) - expenses ( 82 cents )

### Assumption
The Step attempts to deal with a decrease of just a cent but the implementation uses a decrease of 5 cents in price.
This is made for a shorter table :)

### steps 
 1. Definitions
    //Initialise the know
    - expense = 0.8 ie 80 cents in dollars
    - the start price = 2.5
    - tickets = 120
    - min_price = 1.0

    // conditions
    - cent_diff ie hold the diff in cent from 2.5 with the current price
    - ticket_incre_due_to_a_cent_dec: f32 = 15moreticket/5cent decrease

    // declare the unknow
    - define a vector of tuple to how the sales, gain and price
    - gain: f32;
    - sales: f32;
    - expenses: f32;
    - more_tickets:f32

2. calculate the intial value of the sales (price * tickets) expense ie (expense * tickets) and the gain.
3. Store the value of gain price and sales in the vector

4. decrease the price by a cent
5. calculate the diff in cent btwen the price and 2.5
6. Calculate the total number of tickets now
7. Calculate how much a ticket expense would be now

8. repeat 2. to 7. till either gain is < 0 or the price >= min_price

9. for items in the vector, search for that with the has gain



## Main Points 
1. The order of a var in an if login matters `x > y 1= y > x`
2. `iter()` returns an ref without which the vec is moved into the for loop scope, using _iter()_ imply the item should ve deref incase to be assigned



