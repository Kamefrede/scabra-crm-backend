# Scabra CRM

This is a CRM backend done with Rust + Diesel and using Postgres as a database.

This project was done mostly as a learning exercise.

As such, there are many areas that could be improved and I'm welcome to feedback.

# Before running
Generate a secret key for use with Json Web Token Authentication (I personally used https://mkjwk.org/) and place it in the 
src/db folder (Code currently looks for a file named secret.key, could probably be extracted to an env variable)

Be sure to also tweak and create an appropriate .env and Rocket.toml file.

# Credits

Without the following projects it wouldn't be possible for me to make this backend

 * https://github.com/SakaDream/rocket-rest-api-with-jwt
 * https://github.com/diesel-rs/diesel
 * https://github.com/7omb/chrono
 * https://github.com/SergioBenitez/Rocket
 * https://github.com/marcocastignoli/rust_rocket_api_authentication
 * https://github.com/lawliet89/rowdy
 * https://skinkade.github.io/rocket-jwt-roles-demo/
 * https://mmhaskell.com/
 * https://github.com/bevyengine/bevy/blob/master/.cargo/config_fast_builds  
 * And all the other dependencies and sub-dependencies listed in the Cargo.toml

