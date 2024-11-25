# prometheus
This "game engine" will be the core / underlying technology of a platform i want to build in the future</br>
My goals for this and the platform:
* Create and run games
* Have a non-invasive anti-cheat
* Cross-compatible
* Make the platform safe for everyone (ahem)
* Make the platform so easy for developers games are released with high quality and quantity
* Always be open-sourced so "greed" will never come into it
* To learn everything i can about making a large open sourced-project to network technologies to rendering algorithms.


### Hidden Limitations:</br>
* Transparent *instances* need to be spawned **last**.</br> 
  > Since i use instanced rendering it is impossible- that i know of to "sort" the transparent instances before rendering</br>
  > However i *do* sort the objects themselves so they can be added in any order.</br>
  >> My advice is to create a new object when transparency is needed and don't modify the transparency of the instance.</br>