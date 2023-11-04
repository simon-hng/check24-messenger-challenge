export const convertFileListToBase64Array = (fileList: FileList): Promise<string[]> => {
  const base64Array: Promise<string>[] = [];

  for (let i = 0; i < fileList.length; i++) {
    const file = fileList.item(i);

    if (file) {
      const reader = new FileReader();

      base64Array.push(
        new Promise<string>((resolve, reject) => {
          reader.onload = () => {
            if (typeof reader.result === 'string') {
              resolve(reader.result);
            } else {
              reject('Failed to read file as base64.');
            }
          };

          reader.onerror = () => {
            reject('Failed to read file.');
          };

          reader.readAsDataURL(file);
        })
      );
    }
  }

  return Promise.all(base64Array);
}
