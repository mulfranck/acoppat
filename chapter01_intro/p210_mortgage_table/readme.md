# Problem 1.2.12 Mortgage table
Mortgage a form of loan whr the loan is used
to purchase non-movable assets. These asset are legally the lender's till the loan is paid 
```
    i = p * r * ti
    np = p - (pay + i + t))
```
where
    i is the interest calculated from the interest 6% per year
    ti is the time in years: i believe
    p is the principal
    np is the new principal
    t is the tax per month.

## Todo
Will understand in the days coming to make this
- Read in some data from the stdio
- Possible of calculate based on what info is need i.e
    - the interest after a time
    - the pay minimal as with this
    - the count of months for a complete loan refund

## Assumption
if the interest or np is anything <= 0 then the pay that month with be the np + tax

## steps
- Calculate the interest for the month
- Calculate the total pay ie payment + interest + tax
- Calculate the new principal ie the previous principal - total pay
- Draw the row of the calculated month
- update the month to the next month and the principal to equal the new principal
- Till when the interest is 0 is < 1 the then the minimal payment that month is the previous principal + tax

## main points
- `(x * y).round() / y`
run a x to a given dp based on the multiple of 10
```rs
    let x = 12.2345_f32;
    let y = 100.0_f32; // to round to 2 dp

    x = (x * y).round() / y; // 12.23
```
- Mortgage

