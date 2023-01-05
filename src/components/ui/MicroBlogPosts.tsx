import React, { useCallback } from 'react'
import { useEffect } from 'react'

const POSTS_LIMIT = 5

type MicroPost = {
    id: string
    title: string | null
    url: string
    originalSummary: string
    summary: string
    html: string
    date: string
}

function cleanContent(title: string | null, content: string) {
    console.log({ content })
    return (
        content
            // Replace paragraphs with line breaks
            .replace(/<\/p>/g, '\n')
            .replace(/<[^>]*>?/gm, '')
    )
}

function renderPost({ id, url, title, date, summary }: MicroPost) {
    return (
        <div key={id} className="my-4 text content">
            <a href={url} target="_blank" rel="noopener" className="link">
                <time className="" dateTime={date}>
                    {date}
                </time>
                {title && ` - ${title}`}
            </a>

            <p
                className="text my-1"
                dangerouslySetInnerHTML={{ __html: summary }}
            />
        </div>
    )
}

export default function MicroBlogPosts() {
    const [status, setStatus] = React.useState<'loading' | 'error' | 'done'>(
        'loading',
    )
    const [posts, setPosts] = React.useState<MicroPost[]>([])

    const loadPosts = useCallback(async () => {
        try {
            const response = await fetch(
                'https://geekyaubergine.com/feed.json',
            ).then((response) => response.json())

            const { items } = response

            const posts: MicroPost[] = items
                .slice(0, POSTS_LIMIT)
                .map((item) => {
                    const {
                        id,
                        url,
                        content_html,
                        date_published,
                        title = null,
                    } = item

                    return {
                        id,
                        url,
                        title,
                        summary: cleanContent(title, content_html),
                        date: date_published.replace(/T.*/, ''),
                    }
                })

            setPosts(posts)
            setStatus('done')
        } catch (error) {
            console.error(error)
            setStatus('error')
        }
    }, [])

    useEffect(() => {
        loadPosts()
    }, [])

    return (
        <div>
            {status === 'loading' && (
                <p className="my-4">Loading micro blogs</p>
            )}
            {status === 'error' && (
                <p className="my-4">
                    Error loading latest posts. Please visit{' '}
                    <a
                        href="https://geekyaubergine.com"
                        target="_blank"
                        rel="noopener"
                        className="link"
                    >
                        geekyaubergine.com
                    </a>
                </p>
            )}
            {status === 'done' && posts.map(renderPost)}
        </div>
    )
}
