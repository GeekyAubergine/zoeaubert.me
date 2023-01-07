import * as React from 'react'
import { graphql, useStaticQuery } from 'gatsby'
import Helmet from 'react-helmet'
import NavBar from './NavBar'
import Footer from './Footer'
import SEO from '../Seo'

const HTML_ATTRIBUTES = {
    lang: 'en',
}

type Props = {
    title?: string | null
    description?: string | null
    children: React.ReactNode
}
export function Page({ title, description, children }: Props) {
    return (
        <>
            <main className="flex w-full justify-center pt-4 pb-8 px-4 sm:px-8 sm:pt-8">
                <SEO title={title} description={description} />
                <div className="flex flex-col width-control">
                    <NavBar />
                    {children}
                    <Footer />
                </div>
            </main>
        </>
    )
}
