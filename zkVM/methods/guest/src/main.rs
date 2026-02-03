// This guest code will run inside the zkVM and perform the private computation. 
use risc0_zkvm::guest::env;

fn main() {
    // TODO: Implement your guest code here
    // Though Process:
    // I want to prove some one is that age without revealing their actual age
    // Private Input: Since I am using age I will use u32 as the input
    //  - Prove age >= a threshold 18+ so u32 is fine
    // Computation: age >= threshold
    // Public Output: The verifier should see true or false 
    // Control Flow: Checking a single condition so if else is fine
    // Data Type: Scalar (u32) since it represents a single value
    // Compilation will not fail since it compares two u32 values

    // read the input
    let age: u32 = env::read();
    let is_adult: bool = age >= 18;

    // TODO: do something with the input

    // write public output to the journal
    env::commit(&is_adult);
}
