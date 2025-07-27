use leptos::prelude::*;

#[component]
pub fn GetInvolved() -> impl IntoView {
    view! {
        <div id="hero" class="font-serif">
            <div id="image-wrapper" class="flex justify-center">
                <img
                    class="h-[340px] max-h-[340px] max-w-3/4"
                    src="/static/contribute_heroimage.webp"
                    alt="the-hero-image"
                    width="1920px"
                    height="665px"
                />
            </div>

            <div class="absolute top-0 flex h-[340px] w-full justify-center bg-linear-to-b from-sand-100 to-sand-100/30 backdrop-blur-xs">
                <h1 class="mt-30 max-w-[50rem] text-center text-6xl">Contribute to TanakhCC</h1>
            </div>
        </div>
        <div
            id="contribute-wrapper"
            class="flex justify-center bg-linear-to-b from-sand-100/30 to-sand-400/60 p-8"
        >
            <ul class="grid max-w-[50rem] grid-cols-1 gap-12 font-serif text-sand-100">
                <li class="rounded-3xl bg-sand-700 p-6 text-lg">
                    <h2 class="text-center text-4xl">Pray</h2>
                    <div class="flex justify-center">
                        <blockquote class="w-1/2 min-w-72 border-l-8 border-sand-500 bg-sand-200/80 p-4 text-sand-700 m-2">
                            <p class="text-right text-xl">
                                "אִם־יְהוָ֤ה׀ לֹא־יִבְנֶ֬ה בַ֗יִת שָׁ֤וְא׀ עָמְל֣וּ בֹונָ֣יו בֹּ֑ו ׃"
                            </p>
                            <p>
                                When the LORD does not build the house, its builders strive in vain.
                            </p>
                            <p class="text-center">Ps 127:1</p>
                        </blockquote>
                    </div>
                    <p>
                        While TanakhCC deliberately has no specific theological commitment, I firmly believe that this project can only function through Gods great love for his people. Please pray for the success of TanakhCC, and that all dedication poured into it by volunteers bring good fruit for Gods great kingdom.
                    </p>
                </li>
                <li class="rounded-3xl bg-sand-700 p-6 text-lg">
                    <h2 class="text-center text-4xl">Spread the Word</h2>
                    <p>
                        "TanakhCC depends on volunteers to function and produce results in a reasonable timespan. It also depends on many simply using it - our work is not meant to tick checkboxes but to help people. If you know anyone who may be excited about TanakhCC or who may benefit from its data, feel free to share this resource."
                    </p>
                </li>
                <li class="rounded-3xl bg-sand-700 p-6 text-lg">
                    <h2 class="text-center text-4xl">Transcribe</h2>
                    <p>
                        "You know classical(biblical) hebrew and can help transcribe manuscripts? Try it out, every hour of work helps! Transcribing manuscripts does not require specific training and can be picked up quickly by anyone familiar with classical hebrew. Your work will always be compared with the transcriptions of others before being published, so you do not need to worry about making some mistakes! Go ahead and check out "
                        <a
                            class="underline decoration-sand-100 decoration-2"
                            href="https://guide.tanakhcc.org/en"
                        >
                            "the guide to tanakhcc"
                        </a>", where you will get all the information you need to get started."
                    </p>
                </li>
                <li class="rounded-3xl bg-sand-700 p-6 text-lg">
                    <h2 class="text-center text-4xl">Translate</h2>
                    <p>
                        "If you are able to translate this website, "
                        <a
                            href="https://guide.tanakhcc.org/en"
                            class="underline decoration-sand-100 decoration-2"
                        >
                            the guide to tanakhcc
                        </a>", or "
                        <a
                            href="https://critic.tanakhcc.org"
                            class="underline decoration-sand-100 decoration-2"
                        >
                            "critic, our transcription tool"
                        </a>" into a new language, please reach out to "
                        <a
                            href="mailto:info@tanakhcc.org"
                            class="underline decoration-sand-100 decoration-2"
                        >
                            info@tanakhcc.org
                        </a>
                        ". Making TanakhCC available to non-english speakers is a great opportunity to bless even more people with it."
                    </p>
                </li>
                <li class="rounded-3xl bg-sand-700 p-6 text-lg">
                    <h2 class="text-center text-4xl">Develop</h2>
                    <div class="flex justify-center">
                        <a href="https://www.rust-lang.org/" class="m-1 size-28">
                            <img src="/static/logo/rust.svg" />
                        </a>
                        <a class="m-1 size-28" href="https://leptos.dev/">
                            <img src="/static/logo/leptos.svg" />
                        </a>
                        <a href="https://docs.ansible.com/" class="m-1 size-28">
                            <img src="/static/logo/ansible.svg" />
                        </a>

                        <a href="https://www.postgresql.org/" class="size-28 m-1">
                            <img src="/static/logo/postgres.svg" />
                        </a>
                    </div>
                    <p>
                        "You know any of the above tools, and would like to help fix bugs and improve the software at the core of TanakhCC? Head over to "
                        <a
                            href="https://github.com/tanakhcc"
                            class="underline decoration-sand-100 decoration-2"
                        >
                            our github page
                        </a>
                        ", where you will find every line of code required to run TanakhCC, or get in touch at "
                        <a
                            href="mailto:info@tanakhcc.org"
                            class="underline decoration-sand-100 decoration-2"
                        >
                            info@tanakhcc.org
                        </a>.
                    </p>
                </li>
            </ul>
        </div>
    }
}
