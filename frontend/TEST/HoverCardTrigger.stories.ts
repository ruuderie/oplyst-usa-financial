import type { Meta, StoryObj } from '@storybook/vue3';

import HoverCardTrigger from '../components/ui/hover-card/HoverCardTrigger.vue';

const meta = {
  title: 'HoverCardTrigger',
  component: HoverCardTrigger,
  tags: ['autodocs'],
  args: {}, // default value
} satisfies Meta<typeof HoverCardTrigger>;

export default meta;
type Story = StoryObj<typeof HoverCardTrigger>;

export const Primary: Story = {
    args: {
        class: 'page-container__navbar navbar container is-fluid h-28',
    }
};