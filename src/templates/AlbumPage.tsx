import * as React from 'react'
import {graphql} from "gatsby";
import { useKeyDown } from "react-keyboard-input-hook";
import {PHOTO_ALBUM_ALBUMS} from "../../res/photos/albumData";
import Photo from "../components/photo/Photo";
import {PhotoNode} from "../types";
import BasePage from "../components/page/BasePage";
import * as styles from './albumPage.module.scss'

type QueryResponse = {
    allPhoto: {
        edges: PhotoNode[],
    }
}

type Props = {
    data: QueryResponse,
    pageContext: {
        albumUid: string,
    }
}

const CELLS_PER_ROW = 3

type RowCell = {
    landscape: boolean,
    node: PhotoNode,
}

const buildGrid = (nodes: PhotoNode[]): RowCell[][] => {
    const rows: RowCell[][] = [[]]
    let currentRowWidth = 0

    nodes.forEach((node) => {
        const isLandscape = node.node.smallPhoto.childImageSharp.gatsbyImageData.width > node.node.smallPhoto.childImageSharp.gatsbyImageData.height

        const cellWidth = isLandscape ? 2 : 1


        if (currentRowWidth + cellWidth > CELLS_PER_ROW) {
            rows.push([{
                node,
                landscape: isLandscape,
            }])
            currentRowWidth = cellWidth
        } else {
            rows[rows.length - 1].push({
                node,
                landscape: isLandscape,
            })
            currentRowWidth += cellWidth
        }
    })

    return rows
}

const renderRow = (row: RowCell[], renderRowCellCallback: (cell: RowCell) => React.ReactNode) => {
    const key = row.map(c => c.node.node.uid).join('|')
    return (
    <div key={key} className={styles.row}>
        {row.map(renderRowCellCallback)}
    </div>
)
}

const renderGrid = (grid: RowCell[][], renderRow: (row: RowCell[]) => React.ReactNode) => (
    <div className={styles.grid}>
        {grid.map((row) => renderRow(row))}
    </div>
)

const AlbumPage = ({ pageContext, data }: Props) => {
    const { albumUid } = pageContext

    console.log({ data })

    const [openPhotoIndex, setOpenPhotoIndex] = React.useState<number | null>(null)
    const [photoModalOpen, setPhotoModalOpen] = React.useState<boolean>(false)

    const photoOrder = React.useMemo(() => data.allPhoto.edges.map(edge => edge.node.uid), [data])
    const album = PHOTO_ALBUM_ALBUMS[albumUid]
    const grid = React.useMemo(() => buildGrid(data.allPhoto.edges), [data.allPhoto.edges])

    const onPhotoPressedCallback = React.useCallback((photoUid: string) => {
        const index = photoOrder.findIndex((uid) => uid === photoUid)
        if (index >= 0) {
            setOpenPhotoIndex(index)
            setPhotoModalOpen(true)
        }
    }, [])

    const currentPhoto = data.allPhoto.edges[openPhotoIndex]

    const keyDownCallback = React.useCallback(({ keyName }) => {
        if (photoModalOpen) {
            if (keyName === 'ArrowLeft') {
                setOpenPhotoIndex(Math.max(openPhotoIndex - 1, 0))
            } else if (keyName === 'ArrowRight') {
                setOpenPhotoIndex(Math.min(openPhotoIndex + 1, photoOrder.length - 1))
            } else if (keyName === 'Escape') {
                setOpenPhotoIndex(null)
                setPhotoModalOpen(false)
            }
        }
    }, [photoModalOpen, openPhotoIndex])

    useKeyDown(keyDownCallback)

    const renderPhotoCallback = React.useCallback(({ node }: PhotoNode) =>
            <Photo
                key={node.uid}
                photoData={node}
                onPressed={onPhotoPressedCallback}
                className={styles.cell}
            />,
        [onPhotoPressedCallback],
    )

    const renderRowCellCallback = React.useCallback((cell: RowCell) => {
        return renderPhotoCallback(cell.node)
    }, [renderPhotoCallback])

    const renderRowCallback = React.useCallback((row: RowCell[]) => {
        return renderRow(row, renderRowCellCallback)
    }, [renderRowCellCallback])


    if (album == null) {
        return null
    }

    return (
        <BasePage title={album.description} description={album.description}>
            <div className={styles.container}>
                {renderGrid(grid, renderRowCallback)}
            </div>
            {photoModalOpen && (
                <div>
                    {currentPhoto != null && (
                        <Photo
                            photoData={currentPhoto.node}
                            showLarge
                        />
                    )}
                </div>
            )}
        </BasePage>
    )
}

export default AlbumPage

export const pageQuery = graphql`
query MyQuery {
  allPhoto {
    edges {
      node {
        uid
        smallPhoto: localFile {
          childImageSharp {
            gatsbyImageData(width: 200)
          }
        }
        largePhoto: localFile {
          childImageSharp {
            gatsbyImageData(width: 800)
          }
        }
      }
    }
  }
}
`