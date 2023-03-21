/*use profile_learning::kinds::PrimaryColor;
use profile_learning::utils::mix;*/

use profile_learning::PrimaryColor;
use profile_learning::mix;

fn main() {
    let red = PrimaryColor::Red;
    let yellow = PrimaryColor::Yellow;
    mix(red, yellow);
}