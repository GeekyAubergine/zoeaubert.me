import * as React from 'react'
import { Page } from '../components/ui/Page'

const REFERALS = [
    {
        name: 'DigitalOcean',
        description: 'Cloud hosting',
        link: 'https://m.do.co/c/3348c10a29d3',
    },
    {
        name: 'Fathom Analytics',
        description:
            'Cookieless, Privacy focused, GDPR compliant Website analytics',
        link: 'https://usefathom.com/ref/YQXCXP',
    },
]

function renderReferalLink({
    name,
    description,
    link,
}: {
    name: string
    description: string
    link: string
}) {
    return (
        <div className="mb-8">
            <div className="flex flex-row items-baseline text">
                <h3 className="mr-1 text-xl font-normal">{name}</h3>-
                <a href={link} target="_blank" rel="nofollow">
                    <span className="ml-1 link">{link}</span>
                </a>
            </div>
            <p className='secondary'>{description}</p>
        </div>
    )
}

export default function ReferealPage() {
    return (
        <Page>
            <h2 className="pageTitle mb-8">Referrals</h2>
            {REFERALS.map(renderReferalLink)}
        </Page>
    )
}
