import { useEffect, useRef, useState } from 'react'
import { Model } from '../ising_backend/pkg'
import './App.css'

const SIZE = 1600

function App() {
  const model = useRef<Model | null>(null)
  const isPlaying = useRef(false)
  const [fps, setFps] = useState(0)
  const [mspt, setMspt] = useState(0)
  const lastTickedAt = useRef(0)

  useEffect(() => {
    reset()
  }, [])

  const calc = () => {
    if (isPlaying.current) {
      const t1 = performance.now()
      model.current?.update_metropolis()
      model.current?.draw()
      const t2 = performance.now()
      setMspt(t2 - t1)
      if (lastTickedAt.current !== 0) {
        setFps(1000 / (t1 - lastTickedAt.current))
      }
      lastTickedAt.current = t1
      requestAnimationFrame(calc)
    }
  }

  const toggle = () => {
    isPlaying.current = !isPlaying.current
    calc()
  }

  const reset = () => {
    isPlaying.current = false
    model.current?.free()
    model.current = Model.new(SIZE, 0.44, "lattice")
  }

  return (
    <>
      <button type="button" onClick={() => toggle()}>start/stop</button>
      <button type="button" onClick={() => reset()}>reset</button>
      <span>
        fps: {fps.toPrecision(3)} / mspt: {mspt.toPrecision(3)}
      </span>
      <hr />
      <canvas id="lattice"></canvas>
    </>
  )
}

export default App
