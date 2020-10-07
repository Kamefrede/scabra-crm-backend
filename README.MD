# Scabra CRM

This is a CRM backend done with Rust + Diesel and using Postgres as a database.

This project was done mostly as a learning exercise.

As such, there are many areas that could be improved and I'm welcome to feedback.

# Before running
Generate a secret key for use with Json Web Token Authentication (I personally used https://mkjwk.org/) and place it in the 
src/db folder (Code currently looks for a file named secret.key, could probably be extracted to an env variable)

Be sure to also tweak and create an appropriate .env and Rocket.toml file.

