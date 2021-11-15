export type MarkdownRemarkResponse = {
        frontmatter: {
            title: string,
            slug: string,
            categories: string[],
            description: string,
            date: string,
        },
        timeToRead: number,
        html: string,
}

export type MarkdownRemarkNode = {
    node: MarkdownRemarkResponse,
}