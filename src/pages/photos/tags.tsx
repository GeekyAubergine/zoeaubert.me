import { Link } from 'gatsby'
import natsort from 'natsort'
import * as React from 'react'
import { ALBUMS } from '../../../res/photos'
import { Page } from '../../components/ui/Page'

export function renderTag({ name, count }) {
    return (
        <div className='flex items-baseline'>
            <Link to={`/photos/tags/${name}`} className="m-1 ml-0 link no-underline">
                #{name}
            </Link>
            <p className='ml-2'>{count}</p>
        </div>
    )
}

export default function AllPhotos() {
    const tagCounts = React.useMemo(
        () =>
            ALBUMS.reduce((acc: { name: string; count: number }[], album) => {
                const out = acc.slice()

                album.photos.forEach((photo) => {
                    photo.tags.forEach((tag) => {
                        if (!out.find((tc) => tc.name === tag)) {
                            out.push({ name: tag, count: 1 })
                        } else {
                            const t = out.find((tc) => tc.name === tag)
                            if (t != null) {
                                t.count += 1
                            }
                        }
                    })
                })

                return out
            }, []).sort((a, b) => b.count - a.count),
        [],
    )

    return (
        <Page title="Photo Tags">
            <h2 className="text-2xl pt-12 mb-2 font-bold sm:pt-8">
                All Photo tags
            </h2>
            {tagCounts.map(renderTag)}
        </Page>
    )
}
