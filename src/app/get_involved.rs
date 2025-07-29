use leptos::prelude::*;

#[component]
pub fn GetInvolved() -> impl IntoView {
    view! {
    <div id="hero" class="text-white">
          <div id="image-wrapper" class="flex justify-center"><img class="h-[340px] max-h-[340px]" src="https://tanakhcc.org/static/contribute_heroimage.webp" alt="the-hero-image"/></div>
          <div class="absolute top-0 flex h-[340px] w-full justify-center bg-neutral-900/60 backdrop-blur-xs"><h1 class="mt-30 max-w-[50rem] text-center text-6xl font-Inter">Contribute to TanakhCC</h1></div>
        </div>

        <div id="contribute-wrapper" class="flex justify-center p-8 bg-linear-to-b from-neutral-500 to-neutral-800">
          <ul class="text-white grid max-w-[50rem] grid-cols-1 gap-12">
            <li class="rounded-3xl p-6 text-lg bg-neutral-900">
              <h2 class="text-center text-4xl">Pray</h2>
              <div class="flex justify-center">
                <blockquote class="text-neutral-900 m-2 w-1/2 min-w-64 border-l-4 bg-neutral-200 border-neutral-400 p-4">
                  <p class="text-right font-serif text-xl">"אִם־יְהוָ֤ה׀ לֹא־יִבְנֶ֬ה בַ֗יִת שָׁ֤וְא׀ עָמְל֣וּ בֹונָ֣יו בֹּ֑ו ׃"</p>
                  <p>When the LORD does not build the house, its builders strive in vain.</p>
                  <p class="text-center">Ps 127 : 1</p>
                </blockquote>
              </div>
              <p>"While TanakhCC deliberately has no specific theological commitment, I firmly believe that this project can only function through Gods great love for his people. Please pray for the success of TanakhCC, and that all dedication poured into it by volunteers bring good fruit for Gods great kingdom."</p>
            </li>
            <li class="bg-neutral-900 rounded-3xl p-6 text-lg bg-neutral-900">
              <h2 class="text-center text-4xl">Spread the Word</h2>
              <p>"TanakhCC depends on volunteers to function and produce results in a reasonable timespan. It also depends on many simply using it - our work is not meant to tick checkboxes but to help people. If you know anyone who may be excited about TanakhCC or who may benefit from its data, feel free to share this resource."</p>
            </li>
            <li class="bg-neutral-900 rounded-3xl p-6 text-lg bg-neutral-900">
              <h2 class="text-center text-4xl">Transcribe</h2>
              <p>"You know classical(biblical) hebrew and can help transcribe manuscripts? Try it out, every hour of work helps! Transcribing manuscripts does not require specific training and can be picked up quickly by anyone familiar with classical hebrew. Your work will always be compared with the transcriptions of others before being published, so you do not need to worry about making some mistakes! Go ahead and check out "<a class="underline decoration-2" href="https://guide.tanakhcc.org/en">"the guide to tanakhcc"</a>", where you will get all the information you need to get started."</p>
            </li>
            <li class="bg-neutral-900 rounded-3xl p-6 text-lg bg-neutral-900">
              <h2 class="text-center text-4xl">Translate</h2>
              <p>"If you are able to translate this website, "<a href="https://guide.tanakhcc.org/en" class="underline decoration-2">"the guide to tanakhcc"</a>", or "<a href="https://critic.tanakhcc.org" class="underline decoration-2">"critic, our transcription tool"</a>" into a new language, please reach out to "<a href="mailto:info@tanakhcc.org" class="underline decoration-2">"info @ tanakhcc.org"</a>". Making TanakhCC available to non-english speakers is a great opportunity to bless even more people with it."</p>
            </li>
            <li class="bg-neutral-900 rounded-3xl p-6 text-lg bg-neutral-900">
              <h2 class="text-center text-4xl">Develop</h2>
              <div class="flex justify-center">
              <div class="flex sm:flex-row flex-col justify-center">
                <a href="https://www.rust-lang.org/" class="m-1 translate-x-1 translate-y-1 size-28"><img src="https://tanakhcc.org/static/logo/rust.svg"/></a><a class="m-1 size-28" href="https://leptos.dev/"><img src="https://tanakhcc.org/static/logo/leptos.svg" /></a><a href="https://www.postgresql.org/" class="m-1 size-28"><img src="https://tanakhcc.org/static/logo/postgres.svg" /></a>
              </div>
              </div>
              <p>"You know any of the above tools, and would like to help fix bugs and improve the software at the core of TanakhCC? Head over to "<a href="https://github.com/tanakhcc" class="underline decoration-2">"our github page"</a>", where you will find every line of code required to run TanakhCC, or get in touch at "<a href="mailto:info@tanakhcc.org" class="underline decoration-2">"info @ tanakhcc.org"</a>.</p>
            </li>
          </ul>
        </div>
        }
}
