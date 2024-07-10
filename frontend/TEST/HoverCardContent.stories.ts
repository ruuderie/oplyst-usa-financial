import type { Meta, StoryObj } from '@storybook/vue3';

import HoverCardContent from '../components/ui/hover-card/HoverCardContent.vue';

const meta = {
  title: 'HoverCardContent',
  component: HoverCardContent,
  tags: ['autodocs'],
  args: {}, // default value
} satisfies Meta<typeof HoverCardContent>;

export default meta;
type Story = StoryObj<typeof HoverCardContent>;

export const Primary: Story = {
    args: {
        class: 'page-container__navbar navbar container is-fluid h-28',
    }
};