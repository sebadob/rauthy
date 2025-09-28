{{/*
Expand the name of the chart.
*/}}
{{- define "rauthy.name" -}}
{{- default .Chart.Name .Values.nameOverride | trunc 63 | trimSuffix "-" }}
{{- end }}

{{/*
Create a default fully qualified app name.
We truncate at 63 chars because some Kubernetes name fields are limited to this (by the DNS naming spec).
If release name contains chart name it will be used as a full name.
*/}}
{{- define "rauthy.fullname" -}}
{{- if .Values.fullnameOverride }}
{{- .Values.fullnameOverride | trunc 63 | trimSuffix "-" }}
{{- else }}
{{- $name := default .Chart.Name .Values.nameOverride }}
{{- if contains $name .Release.Name }}
{{- .Release.Name | trunc 63 | trimSuffix "-" }}
{{- else }}
{{- printf "%s-%s" .Release.Name $name | trunc 63 | trimSuffix "-" }}
{{- end }}
{{- end }}
{{- end }}

{{/*
Create chart name and version as used by the chart label.
*/}}
{{- define "rauthy.chart" -}}
{{- printf "%s-%s" .Chart.Name .Chart.Version | replace "+" "_" | trunc 63 | trimSuffix "-" }}
{{- end }}

{{/*
Common labels
*/}}
{{- define "rauthy.labels" -}}
helm.sh/chart: {{ include "rauthy.chart" . }}
{{ include "rauthy.selectorLabels" . }}
{{- if .Chart.AppVersion }}
app.kubernetes.io/version: {{ .Chart.AppVersion | quote }}
{{- end }}
app.kubernetes.io/managed-by: {{ .Release.Service }}
{{- end }}

{{/*
Selector labels
*/}}
{{- define "rauthy.selectorLabels" -}}
app.kubernetes.io/name: {{ include "rauthy.name" . }}
app.kubernetes.io/instance: {{ .Release.Name }}
{{- end }}


{{/*
Get a domain name by checking ingress hosts first, then httpRoute hosts, falling back to rauthy.local
*/}}
{{- define "rauthy.domainName" -}}
{{- if and .Values.ingress.enabled .Values.ingress.hosts }}
{{- (index .Values.ingress.hosts 0).host }}
{{- else if and .Values.httpRoute.enabled .Values.httpRoute.hostnames }}
{{- index .Values.httpRoute.hostnames 0 }}
{{- else -}}
rauthy.local
{{- end }}
{{- end }}

{{/*
Assume the external scheme based on service, ingress and httproute configurations
Since there is no way of telling the tls configuration of the gateway, this assumes httpRoute is behind tls
*/}}
{{- define "rauthy.externalScheme" -}}
{{- if or (and .Values.ingress.enabled (gt (len .Values.ingress.tls) 0)) (.Values.httpRoute.enabled) (and (not (eq .Values.service.type "ClusterIP")) (eq .Values.service.scheme "https")) -}}
https
{{- else -}}
http
{{- end -}}
{{- end }}

{{/*
Assume the external port based on service, ingress and httproute configurations
*/}}
{{- define "rauthy.externalPort" -}}
{{- if or (and .Values.ingress.enabled (gt (len .Values.ingress.tls) 0)) (.Values.httpRoute.enabled)  -}}
443
{{- else -}}
{{ .Values.service.port}}
{{- end -}}
{{- end }}

{{/*
Generate the public URL based on the external scheme and domain name
*/}}
{{- define "rauthy.pubUrl" -}}
{{- printf "%s://%s" (include "rauthy.externalScheme" .) (include "rauthy.domainName" .) -}}
{{- end }}

{{/*
Generate rp_origin configuration based on the pubUrl and external port 
*/}}
{{- define "rauthy.rpOrigin" -}}
{{- printf "%s:%s" (include "rauthy.pubUrl" .) (include "rauthy.externalPort" .) -}}
{{- end }}

