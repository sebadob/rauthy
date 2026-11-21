{{- define "rauthy.hostnameByIndex" }}
{{- $ := (index . 0) }}
{{- $i := (index . 1) }}
{{- printf "%s-%d.%s-headless.%s" (include "rauthy.fullname" $) ($i) (include "rauthy.fullname" $) ($.Release.Namespace) }}
{{- end }}

{{- define "rauthy.hostRaftSyncPortsByIndex" }}
{{- $ := (index . 0) }}
{{- include "rauthy.hostnameByIndex" . }}:{{ $.Values.service.hiqliteRaftPort }} {{ include "rauthy.hostnameByIndex" . }}:{{ $.Values.service.hiqliteAPIPort }}
{{- end }}
