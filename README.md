# Cricket Sports Analytics Tool

## What is this?

This repository should serve as the backend for a tool that helps coaches (specifically those for the sport cricket) analyze their players' performance and help them improve.

Here's how it does this:

1. The backend stores all players' performance with respect to some user. This user is the coach. Any retrieval is done with respect to some user, so that every coach can access only their data
2. Using Spice AI's support for Retrieval-Augment-Generation (RAG), the backend enables users to ask complex questions in natural language about their data. This can be especially powerful in a coaching setting where, when creating plans, sifting through heaps of data or writing complex SQL queries is to cumbersome. Using natural language and Spice's support for RAG, this backend simplifies searching for relevant data significantly.
3. This backend also implements vectorized searching through the `notes` entry tied to each score in the database. A coach can search through this column and find when a particular player has performed in a given way in the past quickly. With vectorization, I ensure that results similar to those requested are provided too, effectively going above and beyond in matching the coach's query.

## Setup

**Installations and API Setup**:

- Create a project with Supabase
- Ensure you have an OPEN AI Key
- Install the spice runtime on your system
- Ensure `cargo` is installed

**Environment Setup**: After cloning this repo, ensure your `.env` file looks the following:

```
SUPABASE_KEY=<API KEY LISTED ON SUPABASE>
PROJECT_REF=<AS LISTED ON SUPABASE>
PG_HOST=<HOST LISTED ON SUPABASE>
PG_PORT=<PORT LISTED ON SUPABASE>
PG_USER=<USER LISTED ON SUPABASE>
PG_PASS=<PASSWORD TO PG DB ON SUPABASE>
OPENAI_KEY=<YOUR OPENAI KEY>
```

Also ensure that your OpenAI API key is stored as an environment variable on your system.

**Supabase Setup**:

1. Create a table called `scores`
2. Modify the table policies such that any anonymous and authenticated can make changes to the table
3. Create the following additional columns (leaving `id` and `created_at` as is) with the following types:

| Column Name | Type   |
| ----------- | ------ |
| `name`      | `text` |
| `uid`       | `text` |
| `score`     | `int8` |
| `notes`     | `text` |

## Running the Application

1. Run `spice run`
2. In a separate terminal, run `cargo run`. This should start the server on port 8000.
3. That's it! Feel free to make requests to any of the available end points.

## All Routes

| Path                  | Method | Function                                                | Request Body                              | Response                       |
| --------------------- | ------ | ------------------------------------------------------- | ----------------------------------------- | ------------------------------ |
| `/api/scores/all`     | GET    | Get all scores with the given `uid`                     | `{uid: int}`                              | `Score[]`                      |
| `/api/scores/insert`  | POST   | Insert a score into the database                        | `ScoreFields[]`                           | `{success: bool}`              |
| `/api/scores/query`   | GET    | Query the database with natural language                | `{messages: String[], uid: int}`          | `{response: Stringified Json}` |
| `/api/scores/vsearch` | GET    | Search for a particular entry and those like it quickly | `{text: String, limit: int, uid: String}` | `Score[]`                      |
| `/api/user/signup`    | POST   | Sign Up for an account                                  | `{email: String, password: String}`       | `UserInfo`                     |
| `/api/user/login`     | POST   | Login                                                   | `{email: String, password: String}`       | `UserInfo`                     |
