import { graphql, Link, useStaticQuery } from 'gatsby'
import natsort from 'natsort'
import * as React from 'react'
import { ALBUMS } from '../../../res/photos'
import { Page } from '../../components/ui/Page'

type Result = {
    allAlbumPhoto: {
        edges: {
            node: {
                tags: string[]
            }
        }[]
    }
}

type Tags = Record<string, number>

export function renderTag({ name, count }) {
    return (
        <div className="flex items-baseline">
            <Link
                to={`/photos/tags/${name}`}
                className="m-1 ml-0 link no-underline"
            >
                #{name}
            </Link>
            <p className="ml-2">{count}</p>
        </div>
    )
}

export default function PhotoTags() {
    const result = useStaticQuery<Result>(graphql`
        {
            allAlbumPhoto {
                edges {
                    node {
                        tags
                    }
                }
            }
        }
    `)

    const tagCounts = React.useMemo(() => {
        const tagsMap = result.allAlbumPhoto.edges.reduce<Tags>(
            (acc, { node }) => {
                const { tags } = node

                tags.forEach((tag) => {
                    if (acc[tag]) {
                        acc[tag]++
                    } else {
                        acc[tag] = 1
                    }
                })

                return acc
            },
            {},
        )

        return Object.entries(tagsMap)
            .map(([name, count]) => ({ name, count }))
            .sort((a, b) => b.count - a.count)
    }, [result])

    return (
        <Page title="Photo Tags">
            <h2 className="pageTitle mb-2">Photo Tags</h2>
            {tagCounts.map(renderTag)}
        </Page>
    )
}
