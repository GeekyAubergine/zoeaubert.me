import * as React from 'react'

export default function Footer() {
    return (
        <footer className="flex w-full flex-col justify-center border-t border-t-border mt-12 p-4 sm:flex-row  max-w-[40rem] mx-auto">
            <a
                className="link text-center"
                href="https://zoeaubert.me/rss.xml"
                target="_blank"
                rel="noopener"
            >
             RSS Feed
            </a>
        </footer>
    )
}
