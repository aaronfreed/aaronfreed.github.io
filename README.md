I&rsquo;m a political science and cybersecurity major from Sarasota, FL. Amateur game development, music creation and (re)mastering, and writing consume much of my spare time.

I’m currently heavily involved in developing the following code projects:

* **[VAF](https://github.com/aaronfreed/vasara/)**: A Lua texturing utility for **[*Marathon* Aleph One](https://alephone.lhowon.org/)**. Based on Vasara by **[@Hopper262](https://github.com/Hopper262)** and **Ares Ex Machina**, which in turn is based on **[Visual Mode.lua](https://github.com/treellama/visualmode)** by **[@jonirons](https://github.com/jonirons)** and **[@treellama](https://github.com/treellama)** (Visual Mode.lua is effectively the command line to Vasara&rsquo;s GUI). At the time I began this project, neither plugin had been substantially updated since 2016, so I took it upon myself (with a bit of assistance from **[@SolraBizna](https://github.com/SolraBizna)** and **[@murbruksprodukt](https://github.com/murbruksprodukt)**) to update Vasara to implement new Aleph One features, fix longstanding bugs, and add new features to make texturing easier. This remains a work in progress; when I&rsquo;m satisfied with the results, I may backport some of these changes to Visual Mode.lua. This will only function with Aleph One 1.7 or later, and it is best used with **[Weland](https://github.com/treellama/weland/releases)** (see the readme for an abbreviated setup guide, or my content creation guide directly below for a detailed one).

Pages currently hosted here:

* ***Marathon* / Aleph One Content Creation Guides:**
   * **[Beginners’ guide](https://aaronfreed.github.io/mapmaking101.html)**
      * What editors to use, how to set them up, what guides to follow to get started making levels, what those guides get wrong or omit, and what to do after mastering the basics.
   * **[Advanced guide](https://aaronfreed.github.io/mapmaking.html)**
      * A guide to several intractable problems that weren&rsquo;t documented elsewhere and how to fix them.
   * **The Annotated Forge Manual** (WIP):
      * **[Detailed Table of Contents](https://aaronfreed.github.io/forgemanualcontents.html)**
      * **[Introduction](https://aaronfreed.github.io/forgemanual.html)**
      * **[Tutorial 1](https://aaronfreed.github.io/forgetutorial1.html)** (WIP)
         * Contains numerous annotations correcting errata and filling in the gaps to apply to the modern map editor Weland. I&rsquo;ve only annotated around 25% of it, and the annotations will need substantial amounts of editing before I consider it finished. But it&rsquo;s a start.
   * **[The Annotated Anvil Help Balloons](https://aaronfreed.github.io/anvilhelp.html)** (updated to work as a ShapeFusion reference)
      * Based on the help strings for Anvil, Bungie&rsquo;s official physics, shapes, and sound file editor; with corrections and amendments for its modern replacement ShapeFusion.
   * **[Circles and *Marathon* Mapmaking](marathoncircles.html)**
      * Aleph One can&rsquo;t *exactly* use circles, but it can create rough approximations. I&rsquo;m kind of (in)famous for using them in my maps. I address a few ways to create them and provide some tips for using them effectively, including a table to produce regular polygons with several common side counts and side lengths of 1 World Units.
   * **[Map Complexity: A Case Study](mapcomplexity.html)**
      * Wherein I examine *Eternal*&rsquo;s &ldquo;Where Giants Have Fallen&rdquo;, *Tempus Irae Redux*&rsquo;s &ldquo;Il grande silenzio&rdquo;, *Phoenix*&rsquo;s &ldquo;Stone Temple Pilates&rdquo;, and *Starlight*&rsquo;s &ldquo;Monument to All Your Sins&rdquo; as a demonstration of how much map detail is too much and how much is just enough.
   * **[Reducing Map Index Use](mapindexreduction.html)**
      * What to do if you&rsquo;ve made a map so complex it won&rsquo;t load.
   * **[*Marathon Infinity* Sounds & Sources](marathonsounds.html)**
      * Invaluable information for anyone working with <em>Marathon Infinity</em>&rsquo;s sound file &ndash; or anyone curious where many of its sounds came from.
   * **[Example DefaultNames.txt for ShapeFusion](https://aaronfreed.github.io/defaultnames.html)**
      * Customize almost every ShapeFusion string to match your own scenario&rsquo;s data.
   * **[Where Are Monsters in Marathon...Maps](https://aaronfreed.github.io/marathonmonsters.html)**
      * A case study of the *Marathon* trilogy&rsquo;s monster placement as a microcosm of how restricting variety within individual levels can, counterintuitively, increase variety across an entire game.
   * **[The *Marathon 2* Palette](https://aaronfreed.github.io/marathon2palette.html)**
      * Colors used in vanilla *Marathon 2* and *Marathon Infinity*&rsquo;s 256-color graphics. Forthcoming: the same, but for *Marathon 1*.
   * **[Marathon: Looper](https://aaronfreed.github.io/marathonlooper.html)**
      * How to loop songs in Aleph One prior to 1.7. Split off to its own page because the process it describes is no longer necessary with releases targeting only Aleph One 1.7 or later.
* **Music & Audio**
   * **[My notes on remastering audio](https://aaronfreed.github.io/remastering.html)**
      * An in-depth description of my cleanup process for music and sounds with poor mastering, lossy compression artifacts, or low sample rates, including an explanation of upmastering, a process I typically use in cases where <a href="https://en.wikipedia.org/wiki/Generation_loss" target="_blank">generation loss</a> is a possible concern. (It&rsquo;s impossible to restore data stripped out by digital clipping, lossy compression, or low sample rates; it&rsquo;s only possible to create approximations of what might have been there.) Written for <a href="https://www.izotope.com/en/products/rx.html" target="_blank">iZotope RX</a>, but its principles are usable in any sufficiently powerful audio cleanup software.
   * **[*Marathon* soundtracks](https://aaronfreed.github.io/soundtracks.html)**
      * Remixes and arrangements of Alexander Seropian&rsquo;s soundtrack for <em>Marathon</em> (1994), plus soundtracks for several major <em>Marathon</em> scenarios. I have personally remastered many of these.
   * **[My discography](https://aaronfreed.github.io/discography.html)**
      * Including <em>Marathon</em> content, covers, and original material alike.
   * **[Musical Modes and the Circle of Fifths](https://aaronfreed.github.io/musicalmodes.html)**
      * A lengthy and highly technical examination of how the seven modes of the major scale (Ionian, Dorian, Phrygian, Lydian, Mixolydian, Aeolian, and Locrian) relate to each other and to the circle of fifths (a musical concept inextricably linked with chords and key signatures). Contains a large number of tables illustrating the principle. Also goes into great detail about the ancient Greek scales that gave several of our modes their names (spoiler alert: the ancient &ldquo;Mixolydian&rdquo;, &ldquo;Phrygian&rdquo;, &ldquo;Dorian&rdquo;, and &ldquo;Lydian&rdquo; scales are not the ones bearing those names today).
* **Gaming**
   * **[My portfolio](https://aaronfreed.github.io/portfolio.html)**
      * An overview of my creative output from the past several years, with a heavy focus on <em>Marathon</em> content I&rsquo;ve worked on.
   * **[Hathor&rsquo;s timeline in *Eternal X* 1.3 (major spoilers)](https://aaronfreed.github.io/hathortimeline.html)**
      * Please don&rsquo;t read this unless you&rsquo;ve completed a recent build of *Eternal X* 1.3: it spoils several of the game&rsquo;s major plot twists, and you&rsquo;ll lack the context to grasp their emotional significance.
   * **[*Marathon Istoria*: Conversations with the Dead (major spoilers)](https://aaronfreed.github.io/istoria_body_messages.html)**
      * A catalog of all Yellow Crystal messages in the Aleph One scenario *Istoria*, the keywords used to display them, and the locations of all 28 bodies in the scenario. (Again, please don&rsquo;t read this until you&rsquo;ve finished *Istoria* at least once.)
* **Miscellaneous**
   * **[Trigonometry Tables (Sine, Cosine, and Tangent)](https://aaronfreed.github.io/trigtables.html)**
      * Why? I got sick of throwing them into a calculator app or Google – this way I&rsquo;ll have a URL for them that I&rsquo;ll easily remember.

I&rsquo;ve contributed in varying degrees to the following (mostly) playable game mods. (Dates refer to my involvement with each project rather than its entire development cycle, though in a few cases these are identical.) See the *Marathon* soundtracks page above for links to soundtracks.

* ***[Eternal X](http://eternal.bungie.org/)*** (2018-present): maps, music, sound, writing, scripting, graphics, soundtrack (re)mastering, codirector *(alongside **pfhorrest**, for versions 1.2.1 and later)*
   * [YouTube playlist](https://www.youtube.com/playlist?list=PLoysJW6pXQ6kiy7CaniLiZSRi-5P-SKEN)
   * This scenario remains under active development; a final release of version 1.3 is still forthcoming. We&rsquo;ve released six public previews of 1.3, most recently on 2024-03-07 (with a hotfix on 2024-03-29 to fix a scripting error in the level “Babylon X”), but further revisions to writing, music, and levels are still planned.
* ***[Apotheosis X](https://simplici7y.com/items/apotheosis-x-5)*** (2020-2023): sound, scripting, testing
   * [YouTube playlist](https://www.youtube.com/playlist?list=PLoysJW6pXQ6mOBZaoKUAOqDsfuv-RIneq)
   * [Featured in *PC Gamer*!](https://pcgamer.com/marathon-mod-apotheosis-x)
   * Further updates of *Apotheosis X* have not been ruled out but are not currently under active development.
* ***[Dungeons Presents Hellpak Vol. 1: Not Recommended by Doctors](https://simplici7y.com/items/dungeons-hellpak-vol-1-not-recommended-by-doctors)*** (2020-2024): maps, scripting, soundtrack mastering, writing, graphics, documentation, bug fix releases
   * [YouTube playlist](https://www.youtube.com/playlist?list=PLoysJW6pXQ6kcfNGN3zh0BcsyaLRFkB0k)
   * [Discord server](https://discord.gg/DTMvjFqtTA)
   * This is an iteration of creator tbcr&rsquo;s long-running *Dungeons* show, hence its full title. We released the (hopefully) final bug fix update for this scenario on 2024-06-24.
* ***Trojan*** (2020-2021): sound remastering, soundtrack remastering
   * There are multiple versions of this game for Aleph One:
      * [The first](https://simplici7y.com/items/marathon-trojan/) is a direct conversion of the original game files to Aleph One format by Shappie, with which treellama and I assisted. I did not directly create any content involved with this release.
      * [The second](https://hhas01.itch.io/trojan-se) is a more complicated conversion of the original game files by original creator Hamish Sanderson with some added extras, including my remastered music and sounds. This, unfortunately, does not run on current versions of Aleph One, but you can probably use the remastered music and sounds with Shappie&rsquo;s conversion above.
* ***[Marathon Phoenix](https://simplici7y.com/items/marathon-phoenix-2/)*** (2020, tangentially): soundtrack remastering
   * [Creator RyokoTK&rsquo;s gameplay and commentary](https://www.youtube.com/playlist?list=PL-_EnUuI9PUoIncYlqgWainfOUoZAXhCY)
   * [Additional gameplay by me and others](https://www.youtube.com/playlist?list=PLoysJW6pXQ6k2XxgTocOm9kxC-y6jXAZI)

I&rsquo;m also working on several forthcoming mods:

* ***Dungeons Presents Hellpak Vol. 2: An Exercise in Questionable Taste*** (2023-present, forthcoming): maps, scripting, music, soundtrack mastering, writing
   * [Discord server](https://discord.gg/DTMvjFqtTA)
   * [Preview of music and levels](https://youtu.be/lebTkbIkt5I) (the first song and the first five levels of this video are my work)
   * Map and music submissions for *Vol. 2* closed at the end of 2023; we’re hoping to release it at the end of 2024.
     * Note that we already have more than seven hours of music, which is more than enough for both *Vol. 2* and *Vol. 3*&rsquo;s soundtracks while giving us a decent head start on *Vol. 4*. You’re welcome to get a head start on mapping for *Vol. 3* now, though.
* ***Tempus Irae Redux*** (2020-present, forthcoming): maps, music, sound, scripting, writing, soundtrack mastering
   * [Get the original *Tempus Irae* here](http://nardo.bungie.org/alephone.php)
   * [YouTube playlist](https://www.youtube.com/playlist?list=PLoysJW6pXQ6nVJdLAfq0ZxS6WRv0bC3Fe)
   * I didn&rsquo;t contribute to the original *Tempus Irae* but have been involved in *Redux*&rsquo;s development since it started in roughly May 2020. *Redux* isn&rsquo;t a straight remaster but more of a modern update; since it significantly changes several levels, players may find it interesting to play both. It also isn&rsquo;t planned to supersede the original *Tempus Irae*; the latter&rsquo;s original Aleph One release will continue to be available on Nardo&rsquo;s website after we release *Redux*. We entered a limited beta test in October 2023 and presently aim to release the game in mid-2024 (it might be out by now if I hadn&rsquo;t had a sudden bout of madness and decided to compose several original songs for it).
* ***Where Monsters Are in Dreams*** (2019-present, forthcoming): maps, music, sound, scripting, soundtrack mastering, writing, graphics, codirector *(alongside **CryoS** and **hypersleep**)*
   * [Website](http://bighouse.bungie.org/wmaid/)
   * [YouTube playlist](https://www.youtube.com/playlist?list=PLoysJW6pXQ6kB_7qLbTdgia1hiNOoYmEz)
   * A perpetually in-development scenario (I believe its development stretches back at least as far as 2001). Hypersleep and CryoS are two of the main forces behind *Apotheosis X*; the three of us have recently recommitted to finally finishing this monster (pun intended) and have a detailed, step-by-step roadmap to do so. We&rsquo;re hoping for a late 2024/early 2025 release, but we&rsquo;ve overshot so many estimates that it&rsquo;s become a running gag, so take that with not just a grain or even a boulder of salt but in fact an entire salt mine.
* ***Return to Marathon* Chapter 2** (2023-present, forthcoming): maps, music, sound, scripting, graphics
   * [*Return to Marathon* website](http://bighouse.bungie.org/rtm/)
   * [*Return to Marathon* Chapter 1](https://citadel.lhowon.org/scenarios/return-to-marathon/)
   * I didn&rsquo;t contribute to *Return to Marathon*&rsquo;s first chapter, but I&rsquo;ll be helping to polish up its maps for the release of the full game. Development resumed in August 2023.
* ***Marathon Chronicles*** (1997-present, in progress): maps, writing, sound, music, graphics, soundtrack mastering, director
   * [YouTube playlist](https://www.youtube.com/playlist?list=PLoysJW6pXQ6k1ExcIEZMIOWc9wfgIkYmL)
   * [Most recent public release](https://drive.google.com/open?id=1BtHg2LzQBth25hxB-AURAs8um3yyCaTE), which is now a few years old. I&rsquo;ll release a current build after we release *Tempus Irae Redux*, since *Chronicles* incorporates many of its textures; out of respect for James&rsquo; phenomenal efforts on *Tempus Irae* and *Redux*, I will not be releasing them in any form ahead of *Redux* itself.
   * *Chronicles* is planned to be a sort of grand finale to a loose arc between the fan games *Rubicon*, *Eternal X*, *Phoenix*, *Where Monsters Are in Dreams* (listed in original release order rather than in-universe chronological order). It is planned to resolve Hathor&rsquo;s story, to address some ramifications of *Rubicon*&rsquo;s story, and to resolve the ramifications of the conflict revealed towards the end of *Eternal*.
     However, despite its having technically been in development for nearly two-thirds of my life, it&rsquo;s probably less than half complete. I plan for its second half to be fairly atypical for *Marathon* gameplay, with perhaps dozens of interconnected levels featuring nonlinear objectives and items that enable players to progress in several different ways; they&rsquo;ll be able to obtain these in nearly any order, meaning its objectives will have several possible solutions. None of this is implemented yet, though, and *Chronicles* is currently on the backburner at least until *Tempus Irae Redux* is complete.

In case that wasn&rsquo;t enough, I also run a **[YouTube channel](https://youtube.com/@MarathonVidmaster)** and host a **[repository of Marathon film files](https://github.com/aaronfreed/marathonfilms)**.

All of the above game mods use the **[Aleph One engine](https://alephone.lhowon.org/download.html)**. You may also wish to familiarize yourself with the **[*Marathon* trilogy](https://alephone.lhowon.org/scenarios.html)**; in particular, *Hellpak* runs under *Marathon Infinity*.

I have at least slightly more than cursory knowledge of the following languages (though in a few cases it was years or even decades ago):

* English (obviously)
* Spanish
* Latin
* German
* C#
* Python
* HTML/CSS
* Lua
* Java
* BASIC
* Pascal
* Assembly
* C
* C++
* OpenGL
* Rust

My knowledge of several of these languages is still only *slightly* more than cursory, but in some cases (e.g., Rust, OpenGL, and C++), it is improving. (In C++&rsquo;s case, this is quite begrudging, as every time I find myself writing C++ code, I find myself wishing I were writing Rust or Lua instead.)

Some foci of my original writing include how differences in communication styles and preferences shape people&rsquo;s experiences and perceptions; how our malleable memories and our imperfect awareness of our surroundings lead us to think we know and remember more than we actually do; and the dangers of power and resources becoming overly concentrated in the hands of a few.

You can contact me on Discord at **@Aaron#6608** (or **@aaron6608**). We&rsquo;ll need a server in common if you don&rsquo;t want to wait for me to check my message requests; the **[*Hellpak* Discord](https://discord.gg/DTMvjFqtTA)** and **[*Marathon* Discord](https://discord.gg/NmE66Mhq2a)** are two of your best bets. You can also contact me through my **[YouTube channel](https://youtube.com/@MarathonVidmaster)**. I don&rsquo;t check email often enough for that to be a reliable form of contact.

I&rsquo;m also a member of **[last.fm](http://last.fm/user/Cassandra-Leo)**, **[Rate Your Music](https://rateyourmusic.com/~Cassandra_Leo)**, and several other websites that I check even less often than I check my email; however, those interested in my musical influences may find them enlightening.

Also, in case the insane amount of detail here didn’t make it obvious, I&rsquo;m neurodivergent.

More to come when I feel like updating this further.
