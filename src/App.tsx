import { useEffect, useRef, useState, useId, useCallback } from 'react'
import { Model } from '../ising_backend/pkg'
import './App.css'

const SIZE = 1600

function App() {
  const model = useRef<Model | null>(null)
  const lastTickedAt = useRef(0)

  const [isPlaying, setIsPlaying] = useState(false)
  const [fps, setFps] = useState(0)
  const [mspt, setMspt] = useState(0)

  useEffect(() => {
    if (!isPlaying) {
      return
    }

    let frameId: number

    const tick = () => {
      const t1 = performance.now()
      model.current?.update_metropolis()
      model.current?.draw()
      const t2 = performance.now()
      setMspt(t2 - t1)
      if (lastTickedAt.current !== 0) {
        const newFps = 1000 / (t1 - lastTickedAt.current)
        setFps(prev => (prev + newFps) / 2)
      }
      lastTickedAt.current = t1
      frameId = requestAnimationFrame(tick)
    }

    frameId = requestAnimationFrame(tick)

    return () => cancelAnimationFrame(frameId)
  }, [isPlaying])

  const toggle = () => setIsPlaying(prev => !prev)

  const canvasId = useId()

  const reset = useCallback(() => {
    setIsPlaying(false)
    setFps(0)
    setMspt(0)
    model.current?.free()
    model.current = Model.new(SIZE, 0.44, canvasId)
  }, [canvasId])

  useEffect(reset, [reset])

  return (
    <>
      <button type="button" onClick={toggle}>{isPlaying ? 'stop' : 'start'}</button>
      <button type="button" onClick={reset}>reset</button>
      <span>
        fps: {fps.toFixed(2)} / mspt: {mspt.toFixed(2)}
      </span>
      <hr />
      <canvas id={canvasId}></canvas>
    </>
  )
}

export default App
