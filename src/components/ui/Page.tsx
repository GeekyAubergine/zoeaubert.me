import * as React from 'react'
import NavBar from './NavBar'
import Footer from './Footer'
import SEO from '../Seo'

type Props = {
    children: React.ReactNode
    hideNavBar?: boolean
    hideFooter?: boolean
    widthControlled?: boolean
    mainClassName?: string
}
export function Page({
    children,
    hideNavBar = false,
    hideFooter = false,
    widthControlled = true,
    mainClassName = '',
}: Props) {
    return (
        <main className="flex w-full justify-center">
            <div
                className={`flex flex-col max-w-full mx-auto pt-4 pb-4 px-4 sm:px-0 sm:pt-8 ${
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
