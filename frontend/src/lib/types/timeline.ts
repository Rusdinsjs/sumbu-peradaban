export interface TimelineActor {
  name: string;
  role?: string;
}

export interface TimelineLocation {
  name: string;
  type?: string;
}

export interface TimelineSource {
  id: string;
  title: string;
}

export interface TimelineEvent {
  uuid?: string;
  title: string;
  year: string;
  yearSort: number;
  description: string;
  tier: 'draft' | 'verified' | 'reviewed' | 'canonical';
  actors?: TimelineActor[];
  locations?: TimelineLocation[];
  sources?: TimelineSource[];
  civilization?: string;
  isSynchrnic?: boolean;
}
