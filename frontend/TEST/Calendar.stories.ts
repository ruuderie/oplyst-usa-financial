import type { Meta, StoryObj } from '@storybook/vue3';

import Calendar from '../components/ui/calendar/Calendar.vue';

const meta = {
  title: 'Calendar',
  component: Calendar,
  tags: ['autodocs'],
  args: {}, // default value
} satisfies Meta<typeof Calendar>;

export default meta;
type Story = StoryObj<typeof Calendar>;

export const Primary: Story = {
    args: {
        class: 'page-container__navbar navbar container is-fluid h-28',
    }
};