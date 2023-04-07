package main

import (
	"github.com/disintegration/imaging"
	"golang.org/x/sys/windows"
	"io"
	"io/fs"
	"log"
	"os"
	"os/user"
	"path/filepath"
)

func main() {
	if !onCompatibleWindowsVersion() {
		log.Fatalf("Incompatible Windows version")
	}

	imageDirectory := getImagesDirectory()
	outputDirectory := getAndCreateOutputDirectory()
	images := getImageFiles(imageDirectory)

	err := processFiles(images, outputDirectory)
	if err != nil {
		log.Fatalf("Error while processing images: %s", err)
	}

}

func onCompatibleWindowsVersion() bool {
	_, _, buildNumber := windows.RtlGetNtVersionNumbers()

	return buildNumber >= 14393
}

func getImagesDirectory() string {
	appdata, err := os.UserCacheDir()
	if err != nil {
		log.Fatalf("Unable to retrieve AppData path: %s", err)
	}

	return filepath.Join(appdata, "Packages", "Microsoft.Windows.ContentDeliveryManager_cw5n1h2txyewy", "LocalState", "Assets")
}

func getAndCreateOutputDirectory() string {
	u, err := user.Current()
	if err != nil {
		log.Fatalf("Unable to get current user: %s", err)
	}

	outputPath := filepath.Join(u.HomeDir, "Pictures", "Windows Spotlight")

	if !pathExists(outputPath) {
		err = os.Mkdir(outputPath, 0666)
		if err != nil {
			log.Fatalf("Unable to create output directory: %s", err)
		}
	}

	return outputPath
}

func getImageFiles(path string) []string {
	var images []string
	err := filepath.Walk(path, func(image string, info fs.FileInfo, err error) error {
		if err != nil {
			return err
		}

		if !info.IsDir() {
			images = append(images, image)
		}
		return nil
	})

	if err != nil {
		log.Fatalf("Error while iterating image directory: %s", err)
	}

	return images
}

func processFiles(images []string, outputPath string) error {
	for _, img := range images {
		file, err := os.Open(img)
		if err != nil {
			return err
		}
		defer file.Close()

		i, err := imaging.Decode(file)
		if err != nil {
			return err
		}

		imageBounds := i.Bounds()
		width := imageBounds.Dx()
		height := imageBounds.Dy()

		if width >= 1920 && height >= 1080 {
			_, err = file.Seek(0, io.SeekStart)
			if err != nil {
				return err
			}

			data, err := io.ReadAll(file)
			if err != nil {
				return err
			}

			imageName := filepath.Base(file.Name()) + ".jpg"
			imagePath := filepath.Join(outputPath, imageName)

			if pathExists(imagePath) {
				log.Printf("Image %s has already been saved\n", imageName)
				continue
			}

			err = os.WriteFile(imagePath, data, 0666)
			if err != nil {
				return err
			}

			log.Printf("Saved %s to %s\n", imageName, imagePath)
		}
	}

	return nil
}

func pathExists(path string) bool {
	if _, err := os.Stat(path); err != nil {
		if os.IsNotExist(err) {
			return false
		} else {
			log.Fatalf("Error checking for path existence: %s", err)

			return false
		}
	} else {
		return true
	}
}
