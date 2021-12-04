import "./LayeredImage.css";

interface LayeredImageProps {
  layers: string[];
  width: number;
  height: number;
}

export default function LayeredImage(props: LayeredImageProps) {
  return (
    <div className={"LayeredImage"}>
      {props.layers.map((layer, index) =>
        <img key={`layer-${index}`} alt="" className={"LayeredImage--layer"} src={layer} width={props.width} height={props.height} />
      )}
    </div>
  );
}

//
// export default function LayeredImage(props: LayeredImageProps) {
//   const canvas = useRef<HTMLCanvasElement | null>(null);
//   const ctx = useRef<CanvasRenderingContext2D | null>(null);
//   useEffect(() => {
//     if (canvas.current === null) {
//       return;
//     }
//     ctx.current = canvas.current.getContext('2d');
//     for (let layer in props.layers) {
//       const img = new Image();
//       img.onload = function() {
//         if (ctx.current === null) {
//           return;
//         }
//         ctx.current.drawImage(img, 0, 0, props.width, props.height);
//       };
//     }
//   }, []);
//
//   return (<canvas ref={canvas} width={props.width} height={props.height} />)
// }