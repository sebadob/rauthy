{{/*
Chart name, optionally overridden.
*/}}
{{- define "rauthy.name" -}}
{{- default .Chart.Name .Values.nameOverride | trunc 63 | trimSuffix "-" -}}
{{- end -}}

{{/*
Fully-qualified app name. Kept short and stable because it becomes the
StatefulSet name, the pod hostname prefix, and therefore part of the Raft node
addresses.
*/}}
{{- define "rauthy.fullname" -}}
{{- if .Values.fullnameOverride -}}
{{- .Values.fullnameOverride | trunc 63 | trimSuffix "-" -}}
{{- else -}}
{{- $name := default .Chart.Name .Values.nameOverride -}}
{{- if contains $name .Release.Name -}}
{{- .Release.Name | trunc 63 | trimSuffix "-" -}}
{{- else -}}
{{- printf "%s-%s" .Release.Name $name | trunc 63 | trimSuffix "-" -}}
{{- end -}}
{{- end -}}
{{- end -}}

{{/*
Headless service name used for stable pod DNS and Raft peer discovery.
*/}}
{{- define "rauthy.headlessName" -}}
{{- printf "%s-headless" (include "rauthy.fullname" .) -}}
{{- end -}}

{{/*
Name of the Secret holding config.toml: either the user-provided existing
Secret, or the one this chart creates from `config.content`.
*/}}
{{- define "rauthy.configSecretName" -}}
{{- if .Values.config.existingSecret -}}
{{- .Values.config.existingSecret -}}
{{- else -}}
{{- printf "%s-config" (include "rauthy.fullname" .) -}}
{{- end -}}
{{- end -}}

{{- define "rauthy.chart" -}}
{{- printf "%s-%s" .Chart.Name .Chart.Version | replace "+" "_" | trunc 63 | trimSuffix "-" -}}
{{- end -}}

{{/*
Common labels.
*/}}
{{- define "rauthy.labels" -}}
helm.sh/chart: {{ include "rauthy.chart" . }}
{{ include "rauthy.selectorLabels" . }}
app.kubernetes.io/version: {{ .Values.image.tag | default .Chart.AppVersion | quote }}
app.kubernetes.io/managed-by: {{ .Release.Service }}
{{- end -}}

{{/*
Selector labels. Stable across upgrades - do NOT add version-dependent labels
here, or rolling upgrades of the StatefulSet selector will break.
*/}}
{{- define "rauthy.selectorLabels" -}}
app.kubernetes.io/name: {{ include "rauthy.name" . }}
app.kubernetes.io/instance: {{ .Release.Name }}
{{- end -}}

{{/*
HQL_NODES value: one Raft member per replica, newline-separated, in the format
`<id> <addr_raft> <addr_api>` that Hiqlite parses with `.lines()`. Pod ordinal
i maps to node id i+1, matching Hiqlite's k8s hostname->node-id derivation
(rauthy-0 -> node 1). Addresses use the headless service so peers resolve each
other directly, without a load balancer in between.
*/}}
{{- define "rauthy.hqlNodes" -}}
{{- $fullname := include "rauthy.fullname" . -}}
{{- $headless := include "rauthy.headlessName" . -}}
{{- $nodes := list -}}
{{- range $i := until (int .Values.replicas) -}}
{{- $nodes = append $nodes (printf "%d %s-%d.%s:8100 %s-%d.%s:8200" (add $i 1) $fullname $i $headless $fullname $i $headless) -}}
{{- end -}}
{{- join "\n" $nodes -}}
{{- end -}}
