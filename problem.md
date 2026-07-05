### Compiling all the problems I am facing while building this

1. Choosing a Cron
> A lot of people use traditional Crons, but I decided to use Chronographer because I am an active contributor myself, so it was just a nice fit for me.

2. Randomness while calling AI using Groq
> Groq models are lightweight and good for light tasks, but they tend to give the same answer most of the time. We need a `catalyst` to make it random — it can be anything like a news feed or a certain article. So I am using Hacker News to select a random news item.

3. Not using X's API
> Well, this is just my side project and I don't want to add a new credit card just to test it, so I am using 3rd party APIs instead. This would be okay, but I would not recommend this API for write operations (btw, I am going to use a different account, not going to expose my real account).
