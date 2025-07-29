use leptos::prelude::*;

#[component]
pub fn HomePage() -> impl IntoView {
    view! {
        <div id="hero" class="text-white relative">
          <picture id="image-wrapper" class="absolute h-full -z-30">
            <source srcset="/static/backimage/hero/xl.webp" media="(min-width: 1920px)" />
            <source srcset="/static/backimage/hero/sm.webp" media="(min-width: 895px)" />
            <source srcset="/static/backimage/hero/normal.webp" media="(min-width: 595px)" />
            <img src="/static/backimage/hero/single_row.webp" />
          </picture>
          <div class="flex w-full flex-row justify-center bg-linear-to-b from-neutral-900/70 to-neutral-900/90 pt-28 pb-12">
            <div>
              <h1 class="text-center text-5xl sm:text-7xl font-bold font-Inter">"TanakhCC"</h1>
              <p class="text-center text-3xl sm:text-4xl font-semibold mb-6">"The Hebrew Bible. Open Source."</p>
          <div class="mb-24 sm:mb-32 flex flex-row justify-center"><a href="/read" class="border-white hover:bg-white hover:text-black mx-6 rounded-lg border-4 px-2 text-xl font-semibold">Read Gods Word</a><a href="/get-involved" class="border-white hover:bg-white hover:text-black mx-6 rounded-lg border-4 px-2 text-xl font-semibold">"Join TanakhCC"</a></div>
          <div>
            <p class="max-w-[50rem] min-w-72 text-xl sm:mb-8 px-8">"TanakhCC is a collection of digitized manuscripts of the Hebrew Bible (the Tanakh) made available under the public domain equivalent "<a href="https://creativecommons.org/public-domain/cc0/" class="decoration-neutral-400 underline decoration-2">"CC-0 License"</a>". The volunteers working on TanakhCC provide high quality and transparently reviewed digitizations of the different manuscripts in which the Hebrew Bible is recorded.You may use "<a href="https://github.com/tanakhcc/transcription" class="decoration-neutral-400 underline decoration-2">"this data"</a>" in any way, at any time, for any purpose, all without asking for permission. When the transcriptions are available, they will be used to create a collation, presenting different variants of verses attested across different manuscripts."</p>
          </div>
        </div>
            </div>
          </div>

        <div id="status" class="bg-neutral-900 text-white p-8 py-12">
          <h2 class="mb-8 text-center text-lg">So far there have been</h2>
          <div class="flex justify-center text-lg">
            <div class="grid max-w-[70rem] gap-4 sm:grid-cols-3 sm:gap-12">
              <div class="relative max-w-40">
                <div class="flex flex-col justify-between text-center">
                  <div class="text-3xl sm:text-4xl font-semibold">XXX.XXX</div>
                  <div>words</div><div>transcribed</div>
                </div>
              </div>
              <div class="relative max-w-40">
                <div class="flex flex-col justify-between text-center">
                  <div class="text-3xl sm:text-4xl font-semibold">YY</div>
                  <div>manuscripts</div><div>completed</div>
                </div>
              </div>
              <div class="relative max-w-40">
                <div class="flex flex-col justify-between text-center">
                  <div class="text-3xl sm:text-4xl font-semibold">ZZ</div>
                  <div>active</div><div>volunteers</div>
                </div>
              </div>
          </div>
        </div>
        </div>


    <div id="why-tanakhcc" class="px-20 bg-neutral-900/90 text-white flex justify-center relative">
          <picture id="image-wrapper" class="absolute h-full -z-20">
            <source srcset="/static/backimage/why/xl.webp" media="(min-width: 1920px)" />
            <source srcset="/static/backimage/why/sm.webp" media="(min-width: 895px)" />
            <source srcset="/static/backimage/why/normal.webp" media="(min-width: 595px)" />
            <img src="/static/backimage/why/single_row.webp" />
          </picture>
    <div class="py-6"><h2 class="text-center text-3xl sm:text-4xl min-w-72 max-w-[50rem]">Why TanakhCC ?</h2><p class="text-lg min-w-72 max-w-[50rem]">"Gods word to his people has been preserved through a large set of manuscripts, written down by meticulous scribes centuries before."
                        Despite all efforts, these manuscripts naturally contain small differences.
                        In order to get to the most probable original wording, these manuscripts need to be compared to each other, creating a critical edition.
                        To this date however, there is no true critical edition edition of the Hebrew Bible, and only very few manuscripts have been transcribed - put into digital form.
                        In addition, most editions of the Hebrew Bible are published under copyright and are therefore not truly available.</p><div class="flex justify-center"><blockquote class="w-1/2 min-w-72 border-l-4 border-neutral-400 bg-neutral-600/60 p-4 max-w-[50rem]"><p class="text-2xl text-right font-serif">"נֶֽפֶשׁ־בְּרָכָ֥ה תְדֻשָּׁ֑ן וּ֝מַרְוֶ֗ה גַּם־ה֥וּא יֹורֶֽא ׃"</p><p class="text-lg">The generous will be enriched, and the one who provides water will be satisifed.</p><p class="text-center">Prov 11 : 25</p></blockquote></div><p class="text-lg min-w-72 max-w-[50rem]">says the teacher. So let us provide the water of the word of God generously.</p></div></div>

        <div id="tanakhcc-different" class="flex justify-center py-12 sm:py-20 px-20 bg-neutral-900 text-white">
          <div class="w-[50rem] min-w-72">
            <h2 class="text-center text-3xl sm:text-4xl">How is TanakhCC different from</h2>
            <ul>
              <li>
                <details class="border-neutral-400 my-1 rounded-lg border-2">
                  <summary class="p-2 text-xl">... the Biblia Hebraica series, and the WLC in particular?</summary>
                  <p class="border-neutral-700 border-t p-3 text-lg">"The "<a href="https://www.die-bibel.de/en/biblia-hebraica-stuttgartensia" class="decoration-neutral-300 underline decoration-2">"Biblia Hebraica Stuttgartensia (BHS)"</a>", and the related Westminster Leningrad Codex (WLC) are good reproductions of Codex Leningradensis - the oldest complete manuscript of the entire Hebrew bible and the data contained in the WLC is even "<a href="https://hb.openscriptures.org/" class="decoration-neutral-300 underline decoration-2">"available digitally under CC BY 4.0"</a>". They only represent one manuscript however, providing a diplomatic edition with apparatus containing remarks about other manuscripts instead of giving a well-rounded overview of all available manuscripts. The WLC data is used internally in TanakhCC as a possible base text, which is possible due to its open license."</p>
                </details>
              </li>
              <li>
                <details class="border-neutral-400 my-1 rounded-lg border-2">
                  <summary class="p-2 text-xl">"... the Hebrew University Bible Project ?"</summary>
                  <p class="border-neutral-700 border-t p-3 text-lg">"Simiarly to the BHS, the "<a href="https://openscholar.huji.ac.il/bible_project/hebrew-university-bible-project-hubp" class="decoration-neutral-300 underline decoration-2">"Hebrew University Bible Project (HUBP)"</a>" is a diplomatic edition - this time of the Aleppo Codex. As all the other versions in this list, the HUBP is a top-of-the-line research project and will become a great resource when finished. It also aims to include all variant readings from relevant sources in its apparatus. However, the HUBP neither provides the raw transcription data for those manuscripts, nor does it publish its results under an open license, limiting accessibility."</p>
                </details>
              </li>
              <li>
                <details class="border-neutral-400 my-1 rounded-lg border-2">
                  <summary class="p-2 text-xl">... Miqra al pi ha - Masorah ?</summary>
                  <p class="border-neutral-700 border-t p-3 text-lg">"The Miqra al pi ha-Masorah (MapM) is a diplomatic edition, based on the Aleppo Codex and "<a href="https://docs.google.com/spreadsheets/d/1mkQyj6by1AtBUabpbaxaZq9Z2X3pX8ZpwG91ZCSOEYs/edit?gid=920165745#gid=920165745" class="decoration-neutral-300 underline decoration-2">"available digitally under CC BY SA 4.0"</a>". MapM focusses on transparent documentation of editorial decisions and paying close attention to the Te'amim. Its attention to detail, transparent editorial process and open license make MapM a great resource, free for all to use!"</p>
                </details>
              </li>
              <li>
                <details class="border-neutral-400 my-1 rounded-lg border-2">
                  <summary class="p-2 text-xl">... Hebrew Bible: A Critical Edition?</summary>
                  <p class="border-neutral-700 border-t p-3 text-lg">"The "<a href="https://www.sbl-site.org/sbl-press/browse-books/the-hebrew-bible-a-critical-edition/" class="decoration-neutral-300 underline decoration-2">"Hebrew Bible: A Critical Edition (HBCE)"</a>" is by design an eclectic edition of the Hebrew Bible, combining information from a wide range of manuscripts. It also aims to produce a digital and open-acces critical text."</p>
                </details>
              </li>
            </ul>
          </div>
        </div>
        <div id="how-to-help" class="bg-neutral-900 p-6 px-20">
          <div class="flex justify-center">
            <a href="/get-involved" class="border-4 border-white hover:bg-neutral-900 bg-white hover:text-neutral-100 text-black group card w-[50rem] min-w-72 rounded-4xl p-8"
              ><h2 class="text-center text-3xl sm:text-4xl">How can I contribute ?</h2>
              <p class="text-center text-lg">Find out how you can become a part of TanakhCC!</p>
              <div class="flex justify-end">
                <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" class="size-12"><path stroke-linecap="round" stroke-linejoin="round" d="M13.5 4.5 21 12m0 0-7.5 7.5M21 12H3"></path></svg></div
            ></a>
          </div>
        </div>
    }
}
