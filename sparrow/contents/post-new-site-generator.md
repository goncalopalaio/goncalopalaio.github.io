title = New site generator
date = 10-05-2020

I've got an updated site! It's probably uglier than the previous one but it's now simpler for me to create new posts.
I was using [Hugo](https://gohugo.io/) which is pretty good, but I kinda felt I didn't need all of the features and each time I came back I had to refresh my memory how it worked... so I created a custom site generator.

I have a single folder that contains all the posts and pages in a markdown format. A .md file started with "page-" generates a new html page in the site and adds it to the header. A .md file started with "post-" generates an html with the post, the file must include a line with "title = " and "date = " which from there the title and date of the post is extracted and later added to the html.

I run my generator (written in Rust, not that it really matters). This is pretty simplistic, but allows me to add new features later more easily than if I was using Hugo.

This change was inspired by a post by Fabien Sanglard [0X10 RULES](https://fabiensanglard.net/ilike/index.html).

I think this will allow me to have less friction to create a new blog post. It's simple, but it's custom made for me.