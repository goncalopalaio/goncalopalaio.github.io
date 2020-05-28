title = 💡 Random thought - Making libraries to to reduce bloat
date = 28-05-2020

This is mostly a obvious concept that I thought about while being unable to sleep (it's starting to smell like summer). This isn't a new idea in any way.

In one end, instead of adding features to your applications, create your internal libraries however you like it and add the features to them instead.

I think this is especially useful when you have a large codebase where code duplication is prone to happen.
Make your libraries solid and not depend on thw main application too much. Make a test suite.

On the other end, when there's particular parts of the application that tend to repeat a lot (and I'm thinking about UI adjacent code) promote it into a libray.

Sure, this is super conceptual but it's interesting to think about with the idea of compression oriented programming. Instead of making your code super genric first, you "compress" existing code into a library.


