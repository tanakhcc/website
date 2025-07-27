/// Renders the home page of your application.
use leptos::prelude::*;

#[component]
pub fn HomePage() -> impl IntoView {
    view! {
        <div id="hero">
            <div id="image-wrapper" class="bg-white">
                <img
                    class="h-[340px] max-h-[340px]"
                    src="/static/heroimage.webp"
                    alt="the-hero-image"
                    width="1920px"
                    height="665px"
                />
            </div>
            <div class="absolute top-0 flex w-full flex-row justify-center bg-linear-to-b from-sand-300/20 to-sand-300/70 h-[340px] backdrop-blur-[4px]">
                <div>
                    <div class="m-12 mb-0 flex justify-center">
                        <span class="font-serif text-7xl font-bold">תנך</span>
                        <svg
                            version="1.0"
                            id="Layer_1"
                            xmlns="http://www.w3.org/2000/svg"
                            xmlns:xlink="http://www.w3.org/1999/xlink"
                            class="size-12 translate-x-2 translate-y-4 "
                            viewBox="5.5 -3.5 64 64"
                            enable-background="new 5.5 -3.5 64 64"
                            xml:space="preserve"
                        >
                            <g>
                                <circle class="fill-none" cx="37.785" cy="28.501" r="28.836" />
                                <path d="M37.441-3.5c8.951,0,16.572,3.125,22.857,9.372c3.008,3.009,5.295,6.448,6.857,10.314   c1.561,3.867,2.344,7.971,2.344,12.314c0,4.381-0.773,8.486-2.314,12.313c-1.543,3.828-3.82,7.21-6.828,10.143   c-3.123,3.085-6.666,5.448-10.629,7.086c-3.961,1.638-8.057,2.457-12.285,2.457s-8.276-0.808-12.143-2.429   c-3.866-1.618-7.333-3.961-10.4-7.027c-3.067-3.066-5.4-6.524-7-10.372S5.5,32.767,5.5,28.5c0-4.229,0.809-8.295,2.428-12.2   c1.619-3.905,3.972-7.4,7.057-10.486C21.08-0.394,28.565-3.5,37.441-3.5z M37.557,2.272c-7.314,0-13.467,2.553-18.458,7.657   c-2.515,2.553-4.448,5.419-5.8,8.6c-1.354,3.181-2.029,6.505-2.029,9.972c0,3.429,0.675,6.734,2.029,9.913   c1.353,3.183,3.285,6.021,5.8,8.516c2.514,2.496,5.351,4.399,8.515,5.715c3.161,1.314,6.476,1.971,9.943,1.971   c3.428,0,6.75-0.665,9.973-1.999c3.219-1.335,6.121-3.257,8.713-5.771c4.99-4.876,7.484-10.99,7.484-18.344   c0-3.543-0.648-6.895-1.943-10.057c-1.293-3.162-3.18-5.98-5.654-8.458C50.984,4.844,44.795,2.272,37.557,2.272z M37.156,23.187   l-4.287,2.229c-0.458-0.951-1.019-1.619-1.685-2c-0.667-0.38-1.286-0.571-1.858-0.571c-2.856,0-4.286,1.885-4.286,5.657   c0,1.714,0.362,3.084,1.085,4.113c0.724,1.029,1.791,1.544,3.201,1.544c1.867,0,3.181-0.915,3.944-2.743l3.942,2   c-0.838,1.563-2,2.791-3.486,3.686c-1.484,0.896-3.123,1.343-4.914,1.343c-2.857,0-5.163-0.875-6.915-2.629   c-1.752-1.752-2.628-4.19-2.628-7.313c0-3.048,0.886-5.466,2.657-7.257c1.771-1.79,4.009-2.686,6.715-2.686   C32.604,18.558,35.441,20.101,37.156,23.187z M55.613,23.187l-4.229,2.229c-0.457-0.951-1.02-1.619-1.686-2   c-0.668-0.38-1.307-0.571-1.914-0.571c-2.857,0-4.287,1.885-4.287,5.657c0,1.714,0.363,3.084,1.086,4.113   c0.723,1.029,1.789,1.544,3.201,1.544c1.865,0,3.18-0.915,3.941-2.743l4,2c-0.875,1.563-2.057,2.791-3.541,3.686   c-1.486,0.896-3.105,1.343-4.857,1.343c-2.896,0-5.209-0.875-6.941-2.629c-1.736-1.752-2.602-4.19-2.602-7.313   c0-3.048,0.885-5.466,2.658-7.257c1.77-1.79,4.008-2.686,6.713-2.686C51.117,18.558,53.938,20.101,55.613,23.187z" />
                            </g>
                        </svg>
                    </div>
                    <h1 class="text-center text-8xl font-serif">TanakhCC</h1>
                    <p class="text-center text-3xl">"The Hebrew Bible. Open Source."</p>
                </div>
            </div>
            <div class="absolute top-[260px] w-full flex justify-center">
                <a
                    href="/read"
                    class="mx-1 rounded-lg border-4 border-sand-800 p-1 text-xl font-semibold text-sand-800 hover:bg-sand-800 hover:text-sand-100 focus:bg-sand-800 focus:text-sand-100"
                >
                    "Read Gods Word"
                </a>
                <a
                    href="/get-involved"
                    class="mx-1 rounded-lg border-4 border-sand-800 p-1 text-xl font-semibold text-sand-800 hover:bg-sand-800 hover:text-sand-100 focus:bg-sand-800 focus:text-sand-100"
                >
                    "Join TanakhCC"
                </a>
            </div>
        </div>

        <div
            id="about"
            class="p-6 px-20 bg-linear-to-b to-sand-400/40 from-sand-300/70 font-serif flex justify-center"
        >
            <div>
                <h2 class="text-center text-4xl max-w-[50rem]">What is TanakhCC?</h2>
                <p class="text-lg max-w-[50rem]">
                    "TanakhCC is a collection of digitized manuscripts of the Hebrew Bible (the Tanakh) made available under the public domain equivalent "
                    <a
                        href="https://creativecommons.org/public-domain/cc0/"
                        class="underline decoration-sand-700 decoration-2"
                    >
                        "CC-0 License"
                    </a>.
                    " The volunteers working on TanakhCC provide high quality and transparently reviewed digitizations of the different manuscripts in which the Hebrew Bible is recorded."
                    "You may use "
                    <a
                        href="https://github.com/tanakhcc/transcription"
                        class="underline decoration-sand-700 decoration-2"
                    >
                        "this data"
                    </a>
                    " in any way, at any time, for any purpose, all without asking for permission.
                    When the transcriptions are available, they will be used to create a collation, presenting different variants of verses attested across different manuscripts."
                </p>
            </div>
        </div>

        <div
            id="why-tanakhcc"
            class="p-6 px-20 bg-linear-to-b from-sand-400/40 to-sand-300 font-serif flex justify-center"
        >
            <div>
                <h2 class="text-center text-4xl max-w-[50rem]">Why TanakhCC?</h2>
                <p class="text-lg max-w-[50rem]">
                    "Gods word to his people has been preserved through a large set of manuscripts, written down by meticulous scribes centuries before.
                    Despite all efforts, these manuscripts naturally contain small differences.
                    In order to get to the most probable original wording, these manuscripts need to be compared to each other, creating a critical edition.
                    To this date however, there is no true critical edition edition of the Hebrew Bible, and only very few manuscripts have been transcribed - put into digital form.
                    In addition, most editions of the Hebrew Bible are published under copyright and are therefore not truly available."
                </p>

                <div class="flex justify-center">
                    <blockquote class="w-1/2 min-w-72 border-l-8 border-sand-700 bg-sand-600/60 p-4 max-w-[50rem]">
                        <p class="text-xl text-right">
                            "נֶֽפֶשׁ־בְּרָכָ֥ה תְדֻשָּׁ֑ן וּ֝מַרְוֶ֗ה גַּם־ה֥וּא יֹורֶֽא ׃"
                        </p>
                        <p>
                            "The generous will be enriched, and the one who provides water will be satisifed."
                        </p>
                        <p class="text-center">Prov 11:25</p>
                    </blockquote>
                </div>

                <p class="text-lg max-w-[50rem]">
                    "says the teacher. So let us provide the water of the word of God generously."
                </p>
            </div>
        </div>

        <div id="status" class="bg-sand-600 p-8">
            <h2 class="text-center text-sand-800 text-4xl mb-8">So far there have been...</h2>
            <div class="flex justify-center">
                <div class="grid grid-cols-3 gap-12 max-w-[70rem]">
                    <div class="relative rounded-lg bg-sand-200 text-sand-800 p-4">
                        <p class="text-3xl">
                            <span class=" font-bold">XXXXXX</span>
                            words
                        </p>
                        <p class="text-right">transcribed</p>
                    </div>
                    <div class="relative rounded-lg bg-sand-200 text-sand-800 p-4">
                        <p class="text-3xl">
                            <span class=" font-bold">YY</span>
                            manuscripts
                        </p>
                        <p class="text-right">completed</p>
                    </div>
                    <div class="relative rounded-lg bg-sand-200 text-sand-800 p-4">
                        <p class="text-3xl">
                            <span class=" font-bold">ZZ</span>
                            active
                        </p>
                        <p class="text-right">volunteers</p>
                    </div>
                </div>
            </div>
        </div>
        <div id="tanakhcc-different" class="font-serif flex justify-center bg-sand-300 p-6 px-20">
            <div class="w-[50rem]">
                <h2 class="text-center text-4xl">How is TanakhCC different from...</h2>

                <ul>
                    <li>
                        <details class="rounded-lg border-2 border-sand-700 bg-sand-600/60 my-1">
                            <summary class="text-2xl p-2">
                                "... the Biblia Hebraica series, and the WLC in particular?"
                            </summary>
                            <p class="border-t border-sand-900 p-3 text-lg">
                                "The "
                                <a
                                    href="https://www.die-bibel.de/en/biblia-hebraica-stuttgartensia"
                                    class="underline decoration-sand-800 decoration-2"
                                >
                                    "Biblia Hebraica Stuttgartensia (BHS)"
                                </a>
                                ", and the related Westminster Leningrad Codex (WLC) are good reproductions of Codex Leningradensis - the oldest complete manuscript of the entire Hebrew bible and the data contained in the WLC is even"
                                <a
                                    href="https://hb.openscriptures.org/"
                                    class="underline decoration-sand-800 decoration-2"
                                >
                                    " available digitally under CC BY 4.0."
                                </a>
                                " They only represent one manuscript however, providing a diplomatic edition with apparatus containing remarks about other manuscripts instead of giving a well-rounded overview of all available manuscripts.
                                The WLC data is used internally in TanakhCC as a possible base text, which is possible due to its open license."
                            </p>
                        </details>
                    </li>
                    <li>
                        <details class="rounded-lg border-2 border-sand-700 bg-sand-600/60 my-1">
                            <summary class="text-2xl p-2">
                                ... the Hebrew University Bible Project?
                            </summary>
                            <p class="border-t border-sand-700 p-3 text-lg">
                                "Simiarly to the BHS, the "
                                <a
                                    href="https://openscholar.huji.ac.il/bible_project/hebrew-university-bible-project-hubp"
                                    class="underline decoration-sand-800 decoration-2"
                                >
                                    "Hebrew University Bible Project (HUBP)"
                                </a> " is a diplomatic edition - this time of the Aleppo Codex.
                                As all the other versions in this list, the HUBP is a top-of-the-line research project and will become a great resource when finished.
                                It also aims to include all variant readings from relevant sources in its apparatus.
                                However, the HUBP neither provides the raw transcription data for those manuscripts, nor does it publish its results under an open license, limiting accessibility."
                            </p>
                        </details>
                    </li>
                    <li>
                        <details class="rounded-lg border-2 border-sand-700 bg-sand-600/60  my-1">
                            <summary class="text-2xl p-2">... Miqra al pi ha-Masorah?</summary>
                            <p class="border-t border-sand-700 p-3 text-lg">
                                "The Miqra al pi ha-Masorah (MapM) is a diplomatic edition, based on the Aleppo Codex and "
                                <a
                                    href="https://docs.google.com/spreadsheets/d/1mkQyj6by1AtBUabpbaxaZq9Z2X3pX8ZpwG91ZCSOEYs/edit?gid=920165745#gid=920165745"
                                    class="underline decoration-sand-800 decoration-2"
                                >
                                    "available digitally under CC BY SA 4.0"
                                </a>.
                                " MapM focusses on transparent documentation of editorial decisions and paying close attention to the Te'amim.\n
                                Its attention to detail, transparent editorial process and open license make MapM a great resource, free for all to use!"
                            </p>
                        </details>
                    </li>
                // <li>
                // <details class="rounded-lg border-2 border-sand-700 bg-sand-600/60  my-1">
                // <summary class="text-2xl p-2">
                // "... Hebrew Bible: A Critical Edition?"
                // </summary>
                // <p class="border-t border-sand-700 p-3 text-lg">
                // "The
                // <a href="https://www.sbl-site.org/sbl-press/browse-books/the-hebrew-bible-a-critical-edition/"
                // class="underline decoration-sand-800 decoration-2">
                // Hebrew Bible: A Critical Edition (HBCE)
                // </a>
                // is by design an eclectic edition of the Hebrew Bible, combining information from a wide range of manuscripts.
                // It also aims to produce a digital and open-acces critical text."
                // </p>
                // </details>
                // </li>
                </ul>
            </div>
        </div>

        <div id="how-to-help" class="bg-linear-to-b from-sand-300 to-sand-100 p-6 px-20">
            <div class="flex justify-center">
                <a
                    href="/get-involved"
                    class="w-[50rem] bg-sand-600/60 rounded-4xl p-8 text-sand-800 group card"
                >
                    <h2 class="text-center text-4xl">How can I contribute?</h2>
                    <p class="text-center">
                        Find out how you can become a part of TanakhCC and help provide Gods word
                    </p>
                    <div class="flex justify-end text-sand-600/60 group-hover:text-sand-800">
                        <svg
                            xmlns="http://www.w3.org/2000/svg"
                            fill="none"
                            viewBox="0 0 24 24"
                            stroke-width="1.5"
                            stroke="currentColor"
                            class="size-12"
                        >
                            <path
                                stroke-linecap="round"
                                stroke-linejoin="round"
                                d="M13.5 4.5 21 12m0 0-7.5 7.5M21 12H3"
                            />
                        </svg>
                    </div>
                </a>
            </div>
        </div>
    }
}
