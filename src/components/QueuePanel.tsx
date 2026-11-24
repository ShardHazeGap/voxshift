import { useMemo } from "react";
import { useQueue, Job } from "../hooks/useQueue";

const stateLabel: Record<Job["state"], string> = {
  queued: "Queued",
  analyzing: "Analyzing",
  encoding: "Encoding",
  done: "Done",
  failed: "Failed",
};

export function QueuePanel({ onSelect }: { onSelect: (job: Job) => void }) {
  const { jobs } = useQueue();

  const stats = useMemo(() => {
    const done = jobs.filter((j) => j.state === "done").length;
    const failed = jobs.filter((j) => j.state === "failed").length;
    return { total: jobs.length, done, failed, active: jobs.length - done - failed };
  }, [jobs]);

  if (!jobs.length) {
    return (
      <section className="queue empty">
        <p>Drop files or paste a link to start.</p>
      </section>
    );
  }

  return (
    <section className="queue">
      <header>
        <h2>Queue</h2>
        <span className="stats">
          {stats.active} active · {stats.done} done
          {stats.failed > 0 && ` · ${stats.failed} failed`}
        </span>
      </header>
      <ul>
        {jobs.map((job) => (
          <QueueRow key={job.id} job={job} onClick={() => onSelect(job)} />
        ))}
      </ul>
    </section>
  );
}

function QueueRow({ job, onClick }: { job: Job; onClick: () => void }) {
  const pct = Math.round((job.progress ?? 0) * 100);
  return (
    <li onClick={onClick} className={job.state}>
      <span className="name">{job.source.split(/[\\/]/).pop()}</span>
      <span className="preset">{job.preset}</span>
      <span className="state">{stateLabel[job.state]}</span>
      {job.state === "encoding" && (
        <progress value={pct} max={100} />
      )}
    </li>
  );
}
