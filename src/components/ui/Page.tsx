import * as React from 'react'
import NavBar from './NavBar'
import Footer from './Footer'
import SEO from '../Seo'

type Props = {
    title?: string | null
    description?: string | null
    image?: string | null
    children: React.ReactNode
    hideNavBar?: boolean
    hideFooter?: boolean
    widthControlled?: boolean
    mainClassName?: string
    preventIndexing?: boolean
}
export function Page({
    title,
    description,
    image,
    children,
    hideNavBar = false,
    hideFooter = false,
    widthControlled = true,
    mainClassName = '',
    preventIndexing,
}: Props) {
    return (
        <main className="flex w-full justify-center">
            <SEO
                title={title}
                description={description}
                image={image}
                preventIndexing={preventIndexing}
            />
            <div
                className={`flex flex-col pt-4 pb-4 px-4 sm:px-0 sm:pt-8 ${
                    widthControlled ? 'width-control' : ''
                } ${mainClassName}`}
            >
                {!hideNavBar && <NavBar />}
                {children}
                {!hideFooter && <Footer />}
            </div>
        </main>
    )
}
