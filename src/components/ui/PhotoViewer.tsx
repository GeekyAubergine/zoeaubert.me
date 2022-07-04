import { Link } from 'gatsby'
import * as React from 'react'
import {
    Album,
    ALBUMS,
    albumToSlug,
    Photo as PhotoType,
    PHOTO_CDN_URL,
} from '../../../res/photos'
import Photo from './Photo'

export function renderTag(tag: string) {
    return (
        <Link to={`/photos/tag/${tag}`} className="m-1 link no-underline">
            #{tag}
        </Link>
    )
}

export function PhotoViewer({
    currentPhoto,
    currentAlbum,
    onClose,
    visible,
}: {
    currentPhoto: PhotoType | null
    currentAlbum: Album | null
    onClose: () => void
    visible: boolean
}) {
    return (
        <div
            className={`${
                visible ? 'visible opacity-100' : 'invisible opacity-0'
            } fixed top-0 left-0 right-0 bottom-0 z-10 w-screen h-screen bg-background dark:bg-background-dark flex justify-center items-center transition-all duration-300`}
        >
            {currentPhoto != null && (
                <div>
                    <Photo
                        photo={currentPhoto}
                        className="max-w-[90vw] max-h-[50vh] mb-[25vh] sm:max-h-[70vh] sm:mb-[15vh]"
                    />
                    <div className="fixed top-0 right-0 flex justify-center">
                        <p
                            onClick={onClose}
                            className="cursor-pointer  px-6 py-4"
                        >
                            Close
                        </p>
                    </div>
                    <div className="fixed left-0 right-0 bottom-[1vh] flex justify-center">
                        <div className="flex flex-col justify-start items-start max-w-[90vw] h-[30vh] sm:max-w-[50vw] sm:h-[20vh]">
                            <p className="w-full text-center">
                                {currentPhoto.alt}
                            </p>
                            <div className="flex w-full justify-center items-baseline">
                                {currentAlbum != null && (
                                    <>
                                        <Link
                                            to={albumToSlug(currentAlbum)}
                                            className="text-center link mt-2"
                                            onClick={onClose}
                                        >
                                            Album
                                        </Link>
                                        <p className="mx-2"> - </p>
                                    </>
                                )}
                                <a
                                    className="link"
                                    href={`${PHOTO_CDN_URL}${currentPhoto.url}`}
                                    target="_blank"
                                    rel="noopener"
                                >
                                    Original
                                </a>
                            </div>
                            <div className="flex flex-wrap justify-center mt-2">
                                {currentPhoto.tags.map(renderTag)}
                            </div>
                        </div>
                    </div>
                </div>
            )}
        </div>
    )
}

export function usePhotoViewer({ photos }: { photos: PhotoType[] }) {
    const [open, setOpen] = React.useState(false)
    const [currentPhotoIndex, setCurrentPhotoIndex] = React.useState(0)

    const currentPhoto = photos[currentPhotoIndex]

    const onPhotoClick = React.useCallback(
        (photo) => {
            setCurrentPhotoIndex(photos.indexOf(photo))
            setOpen(true)
        },
        [photos],
    )

    const onClose = React.useCallback(() => {
        setOpen(false)
    }, [])

    const currentAlbum = React.useMemo(
        () =>
            ALBUMS.find((album) =>
                album.photos.find(
                    (photo) =>
                        photo.takenAt === currentPhoto.takenAt &&
                        photo.url === currentPhoto.url,
                ),
            ),
        [currentPhoto],
    )

    const onKeyUp = React.useCallback(
        (event) => {
            if (event.key === 'Escape') {
                setOpen(false)
            }
            if (event.key === 'ArrowLeft') {
                setCurrentPhotoIndex((s) => Math.max(0, s - 1))
            }
            if (event.key === 'ArrowRight') {
                setCurrentPhotoIndex((s) => Math.min(photos.length - 1, s + 1))
            }
        },
        [photos],
    )

    React.useEffect(() => {
        if (typeof window !== 'undefined' && window.document) {
            window.addEventListener('keyup', onKeyUp)
        }
        return () => {
            if (typeof window !== 'undefined' && window.document) {
                window.removeEventListener('keyup', onKeyUp)
            }
        }
    }, [])

    return {
        onPhotoClick,
        Component: (
            <PhotoViewer
                currentPhoto={currentPhoto}
                currentAlbum={currentAlbum}
                onClose={onClose}
                visible={open}
            />
        ),
    }
}
