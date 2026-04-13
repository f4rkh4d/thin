# pulse

a small process supervisor. about 1200 lines. rust.

## why

most supervisors are fine. this one yells at you.

when it comes to restarting a crashed worker, pulse prints a loud red block. it's a bit annoying on purpose.

## use

`pulse run ./my-service`

## notes

i haven't tested on windows. probably works, probably doesn't.
