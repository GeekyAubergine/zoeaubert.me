import { graphql, useStaticQuery } from 'gatsby'
import { PhotoNodeData } from '../res/photos'

export function usePhotoNodeData(): PhotoNodeData[] {
    const allImages = useStaticQuery(
        graphql`
            {
                allFile(filter: { dir: { regex: "/images/" } }) {
                    edges {
                        node {
                            relativePath
                            childImageSharp {
                                gatsbyImageData
                            }
                        }
                    }
                }
            }
        `,
    )

    return allImages.allFile.edges.map((edge) => ({
        relativePath: edge.node.relativePath,
        gatsbyImageData: edge.node.childImageSharp.gatsbyImageData,
    }))
}
